
void * __thiscall
FUN_00463500(void *this,undefined4 param_1,int param_2,uint param_3,int param_4,int param_5,
            int param_6,undefined4 param_7)

{
  undefined4 *puVar1;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063150c;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_004ac120(this,param_1,param_2,param_3,param_4,param_5,param_6,param_7,(uint *)0x0);
  local_4 = 0;
  FUN_004ece30((undefined4 *)((int)this + 0x178));
  local_4._0_1_ = 1;
  *(undefined ***)this = &PTR_FUN_0065a130;
  *(undefined4 *)((int)this + 0x94) = 2;
  *(undefined4 *)((int)this + 0x98) = 7;
  *(uint *)((int)this + 0xb8) = *(uint *)((int)this + 0xb8) | 1;
  *(undefined4 *)((int)this + 0x144) = 0;
  *(undefined4 *)((int)this + 0x14c) = 0;
  *(undefined4 *)((int)this + 0x150) = 0;
  *(undefined4 *)((int)this + 0x158) = 0;
  *(int *)((int)this + 0x154) = param_6;
  puVar1 = (undefined4 *)FUN_00618b70(0x10);
  local_4._0_1_ = 2;
  if (puVar1 == (undefined4 *)0x0) {
    puVar1 = (undefined4 *)0x0;
  }
  else {
    FUN_0060a790(puVar1,2);
    *puVar1 = &PTR_FUN_0065a120;
  }
  local_4._0_1_ = 1;
  *(undefined4 **)((int)this + 0x170) = puVar1;
  puVar1 = (undefined4 *)FUN_00618b70(0x10);
  local_4 = CONCAT31(local_4._1_3_,3);
  if (puVar1 == (undefined4 *)0x0) {
    puVar1 = (undefined4 *)0x0;
  }
  else {
    FUN_0060a790(puVar1,2);
    *puVar1 = &PTR_FUN_0065a120;
  }
  *(undefined4 **)((int)this + 0x174) = puVar1;
  ExceptionList = local_c;
  return this;
}

