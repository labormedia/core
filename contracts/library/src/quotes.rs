use soroban_sdk::{Address, Env, Vec};
use crate::reserves::{get_reserves};

/// given some amount of an asset and pair reserves, returns an equivalent amount of the other asset
// function quote(uint amountA, uint reserveA, uint reserveB) internal pure returns (uint amountB) {
pub fn quote(amount_a: i128, reserve_a: i128, reserve_b: i128) -> i128 {
    //     require(amountA > 0, 'UniswapV2Library: INSUFFICIENT_AMOUNT');
    if amount_a <= 0 {
        panic!("SoroswapLibrary: insufficient amount");
    }
    //     require(reserveA > 0 && reserveB > 0, 'UniswapV2Library: INSUFFICIENT_LIQUIDITY');
    if reserve_a <= 0 || reserve_b <= 0 {
        panic!("SoroswapLibrary: insufficient liquidity");
    }
    //     amountB = amountA.mul(reserveB) / reserveA;
    amount_a.checked_mul(reserve_b).unwrap().checked_div(reserve_a).unwrap()
}


/// given an input amount of an asset and pair reserves, returns the maximum output amount of the other asset
// function getAmountOut(uint amountIn, uint reserveIn, uint reserveOut) internal pure returns (uint amountOut) {
pub fn get_amount_out(amount_in: i128, reserve_in: i128, reserve_out: i128) -> i128 {
    //     require(amountIn > 0, 'UniswapV2Library: INSUFFICIENT_INPUT_AMOUNT');
    if amount_in <= 0 {
        panic!("SoroswapLibrary: insufficient input amount");
    }
    
    //     require(reserveIn > 0 && reserveOut > 0, 'UniswapV2Library: INSUFFICIENT_LIQUIDITY');
    if reserve_in <= 0 || reserve_out <= 0 {
        panic!("SoroswapLibrary: insufficient liquidity");
    }

    //     uint amountInWithFee = amountIn.mul(997);
    let amount_in_with_fee = amount_in.checked_mul(997).unwrap();
    //     uint numerator = amountInWithFee.mul(reserveOut);
    let numerator = amount_in_with_fee.checked_mul(reserve_out).unwrap();

    //     uint denominator = reserveIn.mul(1000).add(amountInWithFee);
    let denominator = reserve_in.checked_mul(1000).unwrap().checked_add(amount_in_with_fee).unwrap();

    //     amountOut = numerator / denominator;
    numerator.checked_div(denominator).unwrap()
}

/// given an output amount of an asset and pair reserves, returns a required input amount of the other asset
// function getAmountIn(uint amountOut, uint reserveIn, uint reserveOut) internal pure returns (uint amountIn) {
pub fn get_amount_in(amount_out: i128, reserve_in: i128, reserve_out: i128) -> i128 {
    //     require(amountOut > 0, 'UniswapV2Library: INSUFFICIENT_OUTPUT_AMOUNT');
    if amount_out <= 0 {
        panic!("SoroswapLibrary: insufficient output amount");
    }
    //     require(reserveIn > 0 && reserveOut > 0, 'UniswapV2Library: INSUFFICIENT_LIQUIDITY');
    if reserve_in <= 0 || reserve_out <= 0 {
        panic!("SoroswapLibrary: insufficient liquidity");
    }
    //     uint numerator = reserveIn.mul(amountOut).mul(1000);
    let numerator = reserve_in.checked_mul(amount_out).unwrap().checked_mul(1000).unwrap();

    //     uint denominator = reserveOut.sub(amountOut).mul(997);
    let denominator = reserve_out.checked_sub(amount_out).unwrap().checked_mul(997).unwrap();

    //     amountIn = (numerator / denominator).add(1);
    numerator.checked_div(denominator).unwrap().checked_add(1).unwrap()
}

/// performs chained getAmountOut calculations on any number of pairs 
// function getAmountsOut(address factory, uint amountIn, address[] memory path) internal view returns (uint[] memory amounts) {
pub fn get_amounts_out(e: Env, factory: Address, amount_in: i128, path: Vec<Address>) -> Vec<i128> {
    //     require(path.length >= 2, 'UniswapV2Library: INVALID_PATH');
    if path.len() < 2 {panic!("SoroswapLibrary: invalid path")};
    
    //     amounts = new uint[](path.length);
    //     amounts[0] = amountIn;
    let mut amounts =  Vec::new(&e);
    amounts.push_back(amount_in);  
    
    //     for (uint i; i < path.length - 1; i++) {
    for i in 0..path.len() - 1 { //  represents a half-open range, which includes the start value (0) but excludes the end value (path.len() - 1)
        // (uint reserveIn, uint reserveOut) = getReserves(factory, path[i], path[i + 1]);
        let (reserve_in, reserve_out) = get_reserves(e.clone(), factory.clone(), path.get(i).unwrap(), path.get(i+1).unwrap());

        // amounts[i + 1] = getAmountOut(amounts[i], reserveIn, reserveOut);
        amounts.push_back(get_amount_out(amounts.get(i).unwrap(), reserve_in, reserve_out))
    }
    amounts
}

/// performs chained getAmountIn calculations on any number of pairs
// function getAmountsIn(address factory, uint amountOut, address[] memory path) internal view returns (uint[] memory amounts) {
pub fn get_amounts_in(e:Env, factory: Address, amount_out: i128, path: Vec<Address>) -> Vec<i128> {
    //     require(path.length >= 2, 'UniswapV2Library: INVALID_PATH');
    if path.len() < 2 {panic!("SoroswapLibrary: invalid path")};

    //     amounts = new uint[](path.length);
    //     amounts[amounts.length - 1] = amountOut;
    let mut amounts =  Vec::new(&e);
    amounts.push_front(amount_out); 

    // TODO: Find a more efficient way to do this
    // for (uint i = path.length - 1; i > 0; i--) {
    for i in (1..path.len()).rev() {
        // (uint reserveIn, uint reserveOut) = getReserves(factory, path[i - 1], path[i]);
        let (reserve_in, reserve_out) = get_reserves( e.clone(), factory.clone(), path.get(i-1).unwrap(), path.get(i).unwrap());
        
        //  amounts[i - 1] = getAmountIn(amounts[i], reserveIn, reserveOut);
        let new_amount = get_amount_in(amounts.get(0).unwrap(), reserve_in, reserve_out);
        // Adds the item to the front.
        // Increases the length by one, shifts all items up by one, and puts the item in the first position.
        amounts.push_front(new_amount)
    }
    amounts
}
