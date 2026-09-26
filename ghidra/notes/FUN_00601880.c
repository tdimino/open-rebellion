
void * __thiscall
FUN_00601880(void *this,char *param_1,int param_2,int param_3,int param_4,int param_5,
            undefined4 param_6,undefined4 param_7)

{
  void *pvVar1;
  uint uVar2;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0065618b;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_00601930(this,param_2,param_3,param_4,param_5,param_6,param_7);
  if (param_1 == (char *)0x0) {
    *(undefined4 *)((int)this + 8) = 0;
  }
  else {
    pvVar1 = (void *)FUN_00618b70(0xc);
    local_4 = 0;
    if (pvVar1 == (void *)0x0) {
      local_4 = 0xffffffff;
      *(undefined4 *)((int)this + 8) = 0;
    }
    else {
      pvVar1 = FUN_005f35b0(pvVar1,param_1);
      local_4 = 0xffffffff;
      *(void **)((int)this + 8) = pvVar1;
    }
  }
  uVar2 = FUN_00603850();
  FUN_00601c60(this,uVar2);
  ExceptionList = local_c;
  return this;
}

