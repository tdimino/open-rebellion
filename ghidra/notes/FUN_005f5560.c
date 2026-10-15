
undefined4 FUN_005f5560(int param_1,int param_2)

{
  undefined4 uVar1;
  
  uVar1 = 0;
  if (((param_1 != 0) && (param_2 != 0)) && (*(uint *)(param_1 + 0x18) < *(uint *)(param_2 + 0x18)))
  {
    uVar1 = 1;
  }
  return uVar1;
}

