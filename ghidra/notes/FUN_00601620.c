
void * __thiscall
FUN_00601620(void *this,ushort param_1,int param_2,int param_3,int param_4,int param_5,int param_6,
            uint param_7,undefined4 param_8,undefined4 param_9)

{
  void *pvVar1;
  CHAR local_10c [256];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0065612e;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_00601930(this,param_2,param_3,param_4,param_5,param_8,param_9);
  FUN_00601c90(this,param_6,(HWND)0x0);
  FUN_00601c60(this,param_7);
  if (param_1 != 0) {
    FUN_00601060((uint)param_1,local_10c,0x100);
    pvVar1 = (void *)FUN_00618b70(0xc);
    local_4 = 0;
    if (pvVar1 == (void *)0x0) {
      pvVar1 = (void *)0x0;
    }
    else {
      pvVar1 = FUN_005f35b0(pvVar1,local_10c);
    }
    *(void **)((int)this + 8) = pvVar1;
  }
  ExceptionList = local_c;
  return this;
}

