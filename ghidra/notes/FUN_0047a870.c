
/* WARNING: Type propagation algorithm not settling */

void * __thiscall FUN_0047a870(void *this,void *param_1)

{
  void *this_00;
  bool bVar1;
  void *pvVar2;
  undefined4 *puVar3;
  uint local_1c [4];
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063351f;
  local_c = ExceptionList;
  local_1c[3] = 0;
  ExceptionList = &local_c;
  FUN_004ece30(local_1c);
  local_4 = 1;
  pvVar2 = FUN_00419a00(*(void **)((int)this + 0x44),0x40000000,0,0x10000000,0,0,0,0x26,(void *)0x1)
  ;
  this_00 = (void *)((int)this + 0x30);
  FUN_00435790(this_00,(int)pvVar2);
  puVar3 = FUN_004357b0(this_00,local_1c + 1);
  local_4._0_1_ = 2;
  FUN_004f26d0(local_1c,puVar3);
  local_4._0_1_ = 1;
  FUN_00619730();
  FUN_005f58b0(this_00);
  pvVar2 = FUN_0041a340(*(void **)((int)this + 0x44),local_1c,0x80000000,0,0x10000000,0,0,0x20000000
                        ,0x21,(void *)0x1);
  FUN_00435790(this_00,(int)pvVar2);
  puVar3 = FUN_004357b0(this_00,local_1c + 1);
  local_4._0_1_ = 3;
  FUN_004f26d0(local_1c,puVar3);
  local_4._0_1_ = 1;
  FUN_00619730();
  FUN_005f58b0(this_00);
  local_1c[1] = 0x90;
  local_1c[2] = 0x98;
  if ((local_1c[0] >> 0x18 < 0x90) || (0x97 < local_1c[0] >> 0x18)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  FUN_00619730();
  if (!bVar1) {
    pvVar2 = FUN_00419a00(*(void **)((int)this + 0x44),0,0,0x10000000,0,0,0,0x26,(void *)0x1);
    FUN_00435790(this_00,(int)pvVar2);
    puVar3 = FUN_004357b0(this_00,local_1c + 1);
    local_4._0_1_ = 4;
    FUN_004f26d0(local_1c,puVar3);
    local_4._0_1_ = 1;
    FUN_00619730();
    FUN_005f58b0(this_00);
    pvVar2 = FUN_0041a340(*(void **)((int)this + 0x44),local_1c,0,0,0x10000000,0,0,0x20000000,0x21,
                          (void *)0x1);
    FUN_00435790(this_00,(int)pvVar2);
    puVar3 = FUN_004357b0(this_00,local_1c + 1);
    local_4._0_1_ = 5;
    FUN_004f26d0(local_1c,puVar3);
    local_4._0_1_ = 1;
    FUN_00619730();
    FUN_005f58b0(this_00);
  }
  FUN_004f26d0(param_1,local_1c);
  local_1c[3] = 1;
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_00619730();
  ExceptionList = local_c;
  return param_1;
}

