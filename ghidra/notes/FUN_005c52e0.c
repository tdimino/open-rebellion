
void * __thiscall
FUN_005c52e0(void *this,undefined4 param_1,undefined4 param_2,undefined4 param_3,int param_4,
            int param_5,undefined4 param_6,undefined4 param_7,uint param_8,uint param_9,
            uint param_10,uint param_11,uint *param_12,int param_13,int param_14)

{
  void *pvVar1;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00653b9e;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_00602150(this,param_1,param_2,param_3,param_4,param_5,param_6,param_7,param_8,param_9,param_12
               ,param_13,param_14);
  local_4 = 0;
  *(undefined ***)this = &PTR_FUN_0066ca60;
  pvVar1 = (void *)FUN_00618b70(0x28);
  local_4._0_1_ = 1;
  if (pvVar1 == (void *)0x0) {
    pvVar1 = (void *)0x0;
  }
  else {
    pvVar1 = FUN_005fbd20(pvVar1,param_1,param_10,10);
  }
  local_4._0_1_ = 0;
  *(void **)((int)this + 0x114) = pvVar1;
  pvVar1 = (void *)FUN_00618b70(0x28);
  local_4 = CONCAT31(local_4._1_3_,2);
  if (pvVar1 == (void *)0x0) {
    pvVar1 = (void *)0x0;
  }
  else {
    pvVar1 = FUN_005fbd20(pvVar1,param_1,param_11,10);
  }
  *(void **)((int)this + 0x118) = pvVar1;
  *(undefined4 *)((int)this + 0x11c) = 1;
  ExceptionList = local_c;
  return this;
}

