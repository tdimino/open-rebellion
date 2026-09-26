
void __thiscall FUN_004913f0(void *this,void *param_1)

{
  undefined4 local_14 [2];
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00635ce8;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  FUN_004f4340(local_14);
  local_4 = 0;
  FUN_004c4c50(this,param_1);
  FUN_005f4d90(param_1,(int)this + 100);
  FUN_005f4d90(param_1,(int)this + 0x7c);
  FUN_005f4d90(param_1,(int)this + 0x80);
  FUN_005f4d90(param_1,(int)this + 0x84);
  FUN_004ece90((void *)((int)this + 0x78),param_1);
  FUN_004ece90((void *)((int)this + 0x74),param_1);
  local_4 = 0xffffffff;
  FUN_004f4380(local_14);
  ExceptionList = pvStack_c;
  return;
}

