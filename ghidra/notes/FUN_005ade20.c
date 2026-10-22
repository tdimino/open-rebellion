
void __thiscall FUN_005ade20(void *this,int param_1)

{
  void *this_00;
  int *piVar1;
  uint uVar2;
  
  uVar2 = *(uint *)(param_1 + 0x178);
  if (uVar2 != 0) {
    this_00 = (void *)FUN_00596bd0();
    piVar1 = (int *)FUN_005a6480(this_00,uVar2);
    FUN_005a8d30(this,param_1,piVar1);
  }
  return;
}

