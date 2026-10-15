
undefined4 FUN_004ae440(int param_1,int param_2)

{
  uint uVar1;
  undefined4 uVar2;
  
  uVar2 = 0;
  if ((((param_2 != 0) && (param_1 != 0)) && (uVar1 = *(uint *)(param_2 + 0x30), (uVar1 & 8) == 0))
     && ((((uVar1 & 2) != 0 && ((uVar1 & 0x200) == 0)) && (0 < *(int *)(param_1 + 0x80))))) {
    uVar2 = 1;
  }
  return uVar2;
}

