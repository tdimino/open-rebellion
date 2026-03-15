
undefined4 __thiscall FUN_00520ec0(void *this,int param_1)

{
  bool bVar1;
  int iVar2;
  undefined3 extraout_var;
  undefined3 extraout_var_00;
  
  iVar2 = FUN_00520b80((int)this);
  if (iVar2 == 0) {
    *(uint *)(param_1 + 4) = *(uint *)(param_1 + 4) & 0xffff0000;
    return 1;
  }
  *(uint *)(param_1 + 4) = *(uint *)(param_1 + 4) & 0xffff4110 | 0x4110;
  bVar1 = FUN_00520ae0((int)this);
  if (CONCAT31(extraout_var,bVar1) == 0) {
    bVar1 = FUN_00520ad0((int)this);
    if (CONCAT31(extraout_var_00,bVar1) != 0) {
      *(uint *)(param_1 + 4) = *(uint *)(param_1 + 4) & 0xffffff00;
      return 1;
    }
  }
  else if (*(int *)((int)this + 100) == 7) {
    *(uint *)(param_1 + 4) = *(uint *)(param_1 + 4) & 0xffffff00 ^ 3;
  }
  return 1;
}

