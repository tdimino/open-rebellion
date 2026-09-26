
void * __thiscall FUN_0041c3d0(void *this,void *param_1)

{
  void *pvVar1;
  undefined4 *puVar2;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062b75b;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_005f5270(this);
  local_4 = 0;
  FUN_004ece30((undefined4 *)((int)this + 0xc));
  pvVar1 = param_1;
  local_4._0_1_ = 1;
  *(undefined ***)this = &PTR_FUN_00658a78;
  puVar2 = FUN_0041c2c0(param_1,&param_1);
  local_4._0_1_ = 2;
  FUN_004f26d0((undefined4 *)((int)this + 0xc),puVar2);
  local_4 = CONCAT31(local_4._1_3_,1);
  FUN_00619730();
  *(undefined4 *)((int)this + 0x10) = *(undefined4 *)((int)pvVar1 + 0x10);
  *(undefined4 *)((int)this + 0x14) = *(undefined4 *)((int)pvVar1 + 0x14);
  ExceptionList = local_c;
  return this;
}

