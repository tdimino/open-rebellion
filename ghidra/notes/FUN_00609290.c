
void __thiscall FUN_00609290(void *this,HWND param_1)

{
  undefined4 *puVar1;
  BOOL BVar2;
  uint uVar3;
  void *pvVar4;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_006567cb;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  if (param_1 != (HWND)0x0) {
    ExceptionList = &pvStack_c;
    BVar2 = IsWindow(param_1);
    if (BVar2 != 0) {
      BVar2 = IsWindowVisible(param_1);
      if (BVar2 != 0) {
        *(HWND *)((int)this + 0x114) = param_1;
      }
    }
  }
  puVar1 = *(undefined4 **)((int)this + 0x104);
  if (puVar1 != (undefined4 *)0x0) {
    FUN_0060db40(puVar1);
    FUN_00618b60((undefined *)puVar1);
  }
  uVar3 = FUN_00600f60(param_1);
  if (uVar3 != 0) {
    pvVar4 = (void *)FUN_00618b70(0x28);
    uStack_4 = 0;
    if (pvVar4 == (void *)0x0) {
      pvVar4 = (void *)0x0;
    }
    else {
      pvVar4 = FUN_0060db00(pvVar4,uVar3);
    }
    *(void **)((int)this + 0x104) = pvVar4;
  }
  ExceptionList = pvStack_c;
  return;
}

