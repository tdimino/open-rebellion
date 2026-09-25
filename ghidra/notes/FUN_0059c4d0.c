
void __thiscall FUN_0059c4d0(void *this,undefined4 *param_1,uint *param_2)

{
  void *pvVar1;
  undefined2 local_14;
  undefined2 local_12;
  void *local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006519e6;
  local_c = ExceptionList;
  local_14 = 0;
  local_12 = DAT_0065d428;
  ExceptionList = &local_c;
  local_10 = (void *)FUN_00618b70(0x114);
  local_4 = 0;
  if (local_10 == (void *)0x0) {
    pvVar1 = (void *)0x0;
  }
  else {
    pvVar1 = FUN_00602150(local_10,*param_1,0x203,0x131,0x1e,0x1a,this,0xad16,0x40a,0x40b,param_2,0,
                          1);
  }
  local_4 = 0xffffffff;
  *(void **)((int)this + 0xa0) = pvVar1;
  if (pvVar1 != (void *)0x0) {
    FUN_006030c0(pvVar1,8);
    FUN_00603150(*(void **)((int)this + 0xa0),4,0x40b);
    local_14 = 0xada6;
    FUN_005ef240(this,*(void **)((int)this + 0xa0),0,(int)&local_14);
  }
  pvVar1 = (void *)FUN_00618b70(0x114);
  local_4 = 1;
  if (pvVar1 == (void *)0x0) {
    pvVar1 = (void *)0x0;
  }
  else {
    pvVar1 = FUN_00602150(pvVar1,*param_1,0x1e3,0x131,0x1e,0x1a,this,0xad17,0x40c,0x40d,param_2,0,0)
    ;
  }
  local_4 = 0xffffffff;
  *(void **)((int)this + 0xa4) = pvVar1;
  if (pvVar1 != (void *)0x0) {
    FUN_006030c0(pvVar1,8);
    FUN_00603150(*(void **)((int)this + 0xa4),4,0x40d);
    local_14 = 0xada8;
    FUN_005ef240(this,*(void **)((int)this + 0xa4),0,(int)&local_14);
  }
  ExceptionList = local_c;
  return;
}

