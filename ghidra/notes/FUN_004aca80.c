
bool __fastcall FUN_004aca80(int *param_1)

{
  void *this;
  undefined4 *puVar1;
  void *local_14 [2];
  void *pvStack_c;
  undefined1 *puStack_8;
  void *local_4;
  
  local_4 = (void *)0xffffffff;
  puStack_8 = &LAB_00638640;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  FUN_004f4340(local_14);
  local_4 = (void *)0x0;
  (**(code **)(*param_1 + 0x58))(local_14,0);
  this = (void *)thunk_FUN_005f5060((int)&stack0xffffffe4);
  if (this != (void *)0x0) {
    puVar1 = FUN_00403040(this,&stack0xffffffe0);
    pvStack_c._0_1_ = 1;
    FUN_004f26d0(local_4,puVar1);
    pvStack_c = (void *)((uint)pvStack_c._1_3_ << 8);
    FUN_00619730();
  }
  pvStack_c = (void *)0xffffffff;
  FUN_004f4380((undefined4 *)&stack0xffffffe4);
  ExceptionList = local_14[0];
  return this != (void *)0x0;
}

