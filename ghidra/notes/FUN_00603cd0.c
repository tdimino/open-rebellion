
void * __thiscall
FUN_00603cd0(void *this,undefined4 param_1,undefined4 param_2,undefined4 param_3,int param_4,
            int param_5,undefined4 param_6,undefined4 param_7,uint param_8,uint param_9,
            uint param_10,uint param_11,uint param_12,uint *param_13,int param_14)

{
  void *pvVar1;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006563da;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_00602150(this,param_1,param_2,param_3,param_4,param_5,param_6,param_7,param_10,param_12,
               param_13,param_14,0);
  local_4 = 0;
  *(undefined ***)this = &PTR_FUN_0066dfc8;
  pvVar1 = (void *)FUN_00618b70(0x28);
  local_4._0_1_ = 1;
  if (pvVar1 == (void *)0x0) {
    pvVar1 = (void *)0x0;
  }
  else {
    pvVar1 = FUN_005fbd20(pvVar1,param_1,param_8,10);
  }
  local_4._0_1_ = 0;
  *(void **)((int)this + 0x114) = pvVar1;
  pvVar1 = (void *)FUN_00618b70(0x28);
  local_4._0_1_ = 2;
  if (pvVar1 == (void *)0x0) {
    pvVar1 = (void *)0x0;
  }
  else {
    pvVar1 = FUN_005fbd20(pvVar1,param_1,param_9,10);
  }
  local_4._0_1_ = 0;
  *(void **)((int)this + 0x118) = pvVar1;
  pvVar1 = (void *)FUN_00618b70(0x28);
  local_4._0_1_ = 3;
  if (pvVar1 == (void *)0x0) {
    pvVar1 = (void *)0x0;
  }
  else {
    pvVar1 = FUN_005fbd20(pvVar1,param_1,param_10,10);
  }
  local_4._0_1_ = 0;
  *(void **)((int)this + 0x11c) = pvVar1;
  pvVar1 = (void *)FUN_00618b70(0x28);
  local_4._0_1_ = 4;
  if (pvVar1 == (void *)0x0) {
    pvVar1 = (void *)0x0;
  }
  else {
    pvVar1 = FUN_005fbd20(pvVar1,param_1,param_11,10);
  }
  local_4._0_1_ = 0;
  *(void **)((int)this + 0x120) = pvVar1;
  pvVar1 = (void *)FUN_00618b70(0x28);
  local_4._0_1_ = 5;
  if (pvVar1 == (void *)0x0) {
    pvVar1 = (void *)0x0;
  }
  else {
    pvVar1 = FUN_005fbd20(pvVar1,param_1,param_12,10);
  }
  *(void **)((int)this + 0x124) = pvVar1;
  *(undefined2 *)((int)this + 0x128) = (undefined2)param_8;
  *(undefined2 *)((int)this + 0x12a) = (undefined2)param_9;
  *(undefined2 *)((int)this + 300) = (undefined2)param_10;
  local_4._0_1_ = 0;
  *(short *)((int)this + 0x12e) = (short)param_11;
  *(undefined2 *)((int)this + 0x130) = (undefined2)param_12;
  *(undefined4 *)((int)this + 0x134) = 2;
  *(undefined4 *)((int)this + 0x138) = 1;
  *(uint **)((int)this + 0x144) = param_13;
  *(undefined4 *)((int)this + 0x13c) = param_2;
  *(undefined4 *)((int)this + 0x140) = param_3;
  pvVar1 = (void *)FUN_00618b70(0x28);
  local_4 = CONCAT31(local_4._1_3_,6);
  if (pvVar1 == (void *)0x0) {
    pvVar1 = (void *)0x0;
  }
  else {
    pvVar1 = FUN_005fbda0(pvVar1,param_4,param_5,param_13);
  }
  *(void **)((int)this + 0x148) = pvVar1;
  ExceptionList = local_c;
  return this;
}

