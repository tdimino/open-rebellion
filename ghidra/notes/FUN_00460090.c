
void * __thiscall
FUN_00460090(void *this,undefined4 param_1,int param_2,uint param_3,int param_4,int param_5,
            int param_6,undefined4 param_7)

{
  void *pvVar1;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00630fc1;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_004ac120(this,param_1,param_2,param_3,param_4,param_5,param_6,param_7,(uint *)0x0);
  local_4 = 0;
  FUN_004ece30((undefined4 *)((int)this + 0x160));
  local_4._0_1_ = 1;
  *(undefined ***)this = &PTR_FUN_0065a000;
  *(undefined4 *)((int)this + 0x94) = 2;
  *(undefined4 *)((int)this + 0x98) = 7;
  *(uint *)((int)this + 0xb8) = *(uint *)((int)this + 0xb8) | 1;
  *(undefined4 *)((int)this + 0x144) = 0;
  *(undefined4 *)((int)this + 0x148) = 0;
  *(undefined4 *)((int)this + 0x14c) = 0;
  *(undefined4 *)((int)this + 0x154) = 0;
  *(int *)((int)this + 0x150) = param_6;
  *(undefined4 *)((int)this + 0x158) = 0;
  pvVar1 = (void *)FUN_00618b70(0x10);
  local_4 = CONCAT31(local_4._1_3_,2);
  if (pvVar1 == (void *)0x0) {
    pvVar1 = (void *)0x0;
  }
  else {
    pvVar1 = FUN_0060a790(pvVar1,2);
  }
  *(void **)((int)this + 0x15c) = pvVar1;
  ExceptionList = local_c;
  return this;
}

