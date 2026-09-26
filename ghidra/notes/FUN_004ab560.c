
void * __thiscall
FUN_004ab560(void *this,undefined4 param_1,undefined4 param_2,int param_3,int param_4,int param_5,
            int param_6,uint param_7,uint param_8,uint param_9,uint param_10,undefined4 param_11,
            void *param_12,void *param_13,void *param_14)

{
  undefined1 uVar1;
  void *pvVar2;
  undefined4 *puVar3;
  void *local_14;
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063841f;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_005f5b80(this);
  *(undefined ***)this = &PTR_FUN_0065bf50;
  local_4._0_1_ = 0;
  local_4._1_3_ = 0;
  *(void **)((int)this + 0x58) = param_13;
  *(void **)((int)this + 0x40) = param_12;
  *(void **)((int)this + 0x5c) = param_14;
  *(undefined4 *)((int)this + 0x74) = 0;
  param_14 = (void *)0x0;
  local_14 = (void *)0x0;
  param_12 = (void *)0x0;
  param_13 = (void *)0x0;
  if ((short)param_7 != 0) {
    pvVar2 = (void *)FUN_00618b70(0x28);
    local_4._0_1_ = 1;
    if (pvVar2 == (void *)0x0) {
      local_14 = (void *)0x0;
    }
    else {
      local_14 = FUN_005fbd20(pvVar2,param_1,param_7,10);
    }
    local_4._0_1_ = 0;
    uVar1 = (undefined1)local_4;
    local_4._0_1_ = 0;
    if ((*(int *)((int)this + 0x58) == 0) || ((short)param_8 == 0)) {
      if ((short)param_9 == 0) {
        param_9 = param_7;
      }
      local_4._0_1_ = uVar1;
      pvVar2 = (void *)FUN_00618b70(0x28);
      local_4._0_1_ = 3;
      if (pvVar2 == (void *)0x0) {
        param_12 = (void *)0x0;
      }
      else {
        param_12 = FUN_005fbd20(pvVar2,param_1,param_9,10);
      }
    }
    else {
      pvVar2 = (void *)FUN_00618b70(0x28);
      local_4._0_1_ = 2;
      if (pvVar2 == (void *)0x0) {
        param_14 = (void *)0x0;
      }
      else {
        param_14 = FUN_005fbd20(pvVar2,param_1,param_8,10);
      }
    }
  }
  local_4._0_1_ = 0;
  if ((short)param_10 != 0) {
    pvVar2 = (void *)FUN_00618b70(0x28);
    local_4._0_1_ = 4;
    if (pvVar2 == (void *)0x0) {
      param_13 = (void *)0x0;
    }
    else {
      param_13 = FUN_005fbd20(pvVar2,param_1,param_10,10);
    }
  }
  local_4._0_1_ = 0;
  pvVar2 = (void *)FUN_00618b70(0xc);
  local_4._0_1_ = 5;
  if (pvVar2 == (void *)0x0) {
    puVar3 = (undefined4 *)0x0;
  }
  else {
    puVar3 = FUN_005f2fc0(pvVar2,param_2);
  }
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_004ab840(this,param_11,param_3,param_4,param_5,param_6,(int)local_14,(int)param_14,
               (int)param_12,(int)param_13,puVar3);
  ExceptionList = local_c;
  return this;
}

