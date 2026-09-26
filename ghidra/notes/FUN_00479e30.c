
void * __thiscall FUN_00479e30(void *this,void *param_1)

{
  void *this_00;
  void *pvVar1;
  undefined4 *puVar2;
  undefined4 local_18;
  undefined1 local_14 [4];
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063343f;
  local_c = ExceptionList;
  local_10 = 0;
  ExceptionList = &local_c;
  FUN_004ece30(&local_18);
  local_4 = 1;
  pvVar1 = FUN_0041a1d0(*(void **)((int)this + 0x44),0x40000000,0,0x40000000,0,0,0,(void *)0x1);
  this_00 = (void *)((int)this + 0x30);
  FUN_00435790(this_00,(int)pvVar1);
  puVar2 = FUN_004357b0(this_00,local_14);
  local_4._0_1_ = 2;
  FUN_004f26d0(&local_18,puVar2);
  local_4._0_1_ = 1;
  FUN_00619730();
  FUN_005f58b0(this_00);
  pvVar1 = FUN_0041a460(*(void **)((int)this + 0x44),&local_18,0x80000000,0,0x40000000,0,0,0,
                        (void *)0x1);
  FUN_00435790(this_00,(int)pvVar1);
  puVar2 = FUN_004357b0(this_00,local_14);
  local_4._0_1_ = 3;
  FUN_004f26d0(&local_18,puVar2);
  local_4._0_1_ = 1;
  FUN_00619730();
  FUN_005f58b0(this_00);
  FUN_004f26d0(param_1,&local_18);
  local_10 = 1;
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_00619730();
  ExceptionList = local_c;
  return param_1;
}

