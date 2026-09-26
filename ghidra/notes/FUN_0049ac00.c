
void * __thiscall FUN_0049ac00(void *this,int param_1,int param_2,uint param_3)

{
  undefined4 uVar1;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00636cb9;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_004c4d90(this,param_2);
  local_4 = 0;
  FUN_004ece30((undefined4 *)((int)this + 0x44));
  local_4._0_1_ = 1;
  FUN_004ece30((undefined4 *)((int)this + 0x48));
  local_4._0_1_ = 2;
  FUN_004ece30((undefined4 *)((int)this + 0x4c));
  local_4._0_1_ = 3;
  FUN_004f5510((undefined4 *)((int)this + 0x60));
  local_4._0_1_ = 4;
  FUN_004f5510((undefined4 *)((int)this + 0x68));
  local_4._0_1_ = 5;
  FUN_004f5510((undefined4 *)((int)this + 0x70));
  local_4._0_1_ = 6;
  FUN_004f5510((undefined4 *)((int)this + 0x78));
  local_4._0_1_ = 7;
  FUN_004f5510((undefined4 *)((int)this + 0x80));
  local_4._0_1_ = 8;
  FUN_005f2f50((undefined4 *)((int)this + 0x88));
  local_4._0_1_ = 9;
  FUN_005f2f50((undefined4 *)((int)this + 0x94));
  local_4._0_1_ = 10;
  FUN_005f2f50((undefined4 *)((int)this + 0xa0));
  local_4._0_1_ = 0xb;
  FUN_005f2f50((undefined4 *)((int)this + 0xac));
  local_4._0_1_ = 0xc;
  FUN_005f2f50((undefined4 *)((int)this + 0xb8));
  local_4 = CONCAT31(local_4._1_3_,0xd);
  *(undefined ***)this = &PTR_FUN_0065bc08;
  uVar1 = FUN_0048bc60();
  *(undefined4 *)((int)this + 0x34) = uVar1;
  *(undefined4 *)((int)this + 0x50) = 0;
  *(undefined4 *)((int)this + 0x54) = 0;
  if (param_2 == 1) {
    *(undefined4 *)((int)this + 0x58) = 1;
  }
  else if (param_2 == 2) {
    *(undefined4 *)((int)this + 0x58) = 2;
  }
  else {
    *(undefined4 *)((int)this + 0x58) = 0;
  }
  FUN_0049b720(this,param_3);
  uVar1 = FUN_0049b0f0(this,param_1);
  *(undefined4 *)((int)this + 0x1c) = uVar1;
  ExceptionList = local_c;
  return this;
}

