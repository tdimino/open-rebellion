
void __thiscall FUN_0043ef10(int param_1,int *param_2)

{
  undefined4 uVar1;
  void *unaff_ESI;
  int local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062e418;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  local_10 = param_1;
  uVar1 = FUN_0043f110(&local_10);
  local_4 = 0;
  (**(code **)(*param_2 + 0x170))(param_1 + 0x20,uVar1);
  pvStack_c = (void *)0xffffffff;
  FUN_00619730();
  ExceptionList = unaff_ESI;
  return;
}

