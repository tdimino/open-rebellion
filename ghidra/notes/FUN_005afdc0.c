
void FUN_005afdc0(void *param_1)

{
  int iVar1;
  
  FUN_005a6660(param_1,1);
  *(undefined4 *)((int)param_1 + 0x20) = *(undefined4 *)((int)param_1 + 0x18);
  iVar1 = FUN_00596bd0();
  *(void **)(iVar1 + 0x44) = param_1;
  return;
}

