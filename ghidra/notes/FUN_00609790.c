
void * __thiscall
FUN_00609790(void *this,undefined4 param_1,short param_2,short param_3,char *param_4,int param_5,
            undefined4 param_6)

{
  void *pvVar1;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0065682b;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  pvVar1 = (void *)FUN_00618b70(0x68);
  local_4 = 0;
  if (pvVar1 == (void *)0x0) {
    pvVar1 = (void *)0x0;
  }
  else {
    pvVar1 = FUN_0060ba00(pvVar1,param_1);
  }
  *(undefined4 *)((int)pvVar1 + 0x54) = param_6;
  local_4 = 0xffffffff;
  FUN_0060bd20(pvVar1,param_2,param_3,*(undefined4 *)((int)this + 0x1c));
  FUN_005f35e0((void *)((int)pvVar1 + 0x14),param_4);
  FUN_005f59f0(*(void **)((int)this + 0xa0),(int)pvVar1);
  FUN_00609ae0(this);
  if (param_5 != 0) {
    FUN_00609a00(this);
  }
  ExceptionList = local_c;
  return pvVar1;
}

