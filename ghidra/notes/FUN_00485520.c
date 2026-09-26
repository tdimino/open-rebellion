
void FUN_00485520(int param_1)

{
  undefined4 *puVar1;
  int *piVar2;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063475f;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  puVar1 = (undefined4 *)FUN_00618b70(0x30);
  if (puVar1 == (undefined4 *)0x0) {
    FUN_00619730();
  }
  local_4 = 0;
  if (puVar1 == (undefined4 *)0x0) {
    piVar2 = (int *)0x0;
  }
  else {
    piVar2 = FUN_00440c60(puVar1);
  }
  local_4 = 0xffffffff;
  if (piVar2 != (int *)0x0) {
    piVar2[0xb] = *(int *)(param_1 + 0x18);
    piVar2[8] = *(int *)(param_1 + 0x20);
    FUN_004855b0(piVar2);
    (**(code **)*piVar2)(1);
  }
  ExceptionList = local_c;
  return;
}

