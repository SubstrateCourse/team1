pragma solidity ^0.6.0;

import "./SafeMath.sol";

abstract contract ERC20 {
   function totalSupply() public virtual view returns (uint256);
   function balanceOf(address owner) public virtual view returns (uint256);
   function transfer(address to, uint256 value) public virtual returns (bool);
   function transferFrom(address from, address to, uint256 value) public virtual returns (bool);
   function approve(address spender, uint256 value) public virtual returns (bool);
   function allowance(address owner, address spender) public virtual view returns (uint256);
   event Transfer(address indexed from, address indexed to, uint256 value);
   event Approval(address indexed owner, address indexed spender, uint256 value);
}

contract xyc is ERC20 {

    using SafeMath for uint256;

    string name = "XuanYuan Coin";
    string symbol = "xyc";
    uint8 decimals;
    uint256 private _totalSupply;
    mapping(address => uint256) private _balances;
    mapping(address => mapping(address => uint256)) private _allowances;

    constructor(uint256 myTotalSupply, uint8 mydecimals) public {
        _totalSupply = myTotalSupply;
        decimals = mydecimals;
        _balances[msg.sender] = myTotalSupply;
    }

   function totalSupply() public override view returns (uint256) {
       return _totalSupply;
   }

   function balanceOf(address _owner) public override view returns (uint256) {
       return _balances[_owner];
   }

   function transfer(address to, uint256 value) public override returns (bool) {
       require(to != address(0), "ERC20 ERROR: transfer from zero address");
       require(value <= _balances[msg.sender]);
       require(_balances[to] + value >= _balances[to]);

       _balances[msg.sender] = _balances[msg.sender].sub(value);
       _balances[to] = _balances[to].add(value);

       emit Transfer(msg.sender, to, value);

       return true;
   }

   function transferFrom(address from, address to, uint256 value) public override returns (bool) {
       require(to != address(0));
       require(value <= _balances[from]);
       require(value <= _allowances[from][msg.sender]);
       require(_balances[to] + value >= _balances[to]);

       _balances[from] = _balances[from].sub(value);
       _balances[to] = _balances[to].add(value);
       _allowances[from][msg.sender] = _allowances[from][msg.sender].sub(value);

       emit Transfer(from, to, value);

       return true;
   }

   function approve(address spender, uint256 value) public override returns (bool) {
       require(value <= _balances[msg.sender]);
       _allowances[msg.sender][spender] = value;
       emit Approval(msg.sender, spender, value);
       return true;
   }

   function allowance(address _owner, address spender) public override view returns (uint256) {
       return _allowances[_owner][spender];
   }

   event Transfer(address indexed from, address indexed to, uint256 value);
   event Approval(address indexed owner, address indexed spender, uint256 value);
}
