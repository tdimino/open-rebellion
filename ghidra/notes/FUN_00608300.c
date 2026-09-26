
void __thiscall FUN_00608300(void *this,uint *param_1,int param_2,uint param_3)

{
  HDC hDC;
  void *pvVar1;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_0065677b;
  local_c = ExceptionList;
  if (param_1 != (uint *)0x0) {
    ExceptionList = &local_c;
    if (*(int *)((int)this + 0xa8) == 0) {
      ExceptionList = &local_c;
      hDC = GetDC(*(HWND *)((int)this + 0x18));
      pvVar1 = (void *)FUN_00618b70(0x28);
      uStack_4 = 0;
      if (pvVar1 == (void *)0x0) {
        pvVar1 = (void *)0x0;
      }
      else {
        pvVar1 = FUN_005fbea0(pvVar1,*(LONG *)((int)this + 0x38),*(LONG *)((int)this + 0x3c),param_1
                              ,hDC);
      }
      *(void **)((int)this + 0xa8) = pvVar1;
      uStack_4 = 0xffffffff;
      ReleaseDC(*(HWND *)((int)this + 0x18),hDC);
    }
    FUN_005fcc30(param_1,*(uint **)((int)this + 0xa8),0,0,param_2,param_3,(undefined4 *)0x0,0);
  }
  ExceptionList = local_c;
  return;
}

