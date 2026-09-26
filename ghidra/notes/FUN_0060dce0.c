
void * __fastcall FUN_0060dce0(undefined4 *param_1)

{
  LONG LVar1;
  LONG LVar2;
  uint uVar3;
  void *pvVar4;
  HDC hdc;
  HPALETTE hPal;
  void *pvVar5;
  int iVar6;
  int iVar7;
  uint *puVar8;
  HDC pHVar9;
  int aiStack_14 [2];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_00656c56;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  uVar3 = FUN_0060dba0((int)param_1);
  if (uVar3 == 0) {
    puVar8 = (uint *)*param_1;
    LVar1 = *(LONG *)(param_1[1] + 0x30);
    LVar2 = *(LONG *)(param_1[1] + 0x34);
    param_1[9] = param_1[9] | 1;
    uVar3 = GetWindowLongA(*(HWND *)(param_1[1] + 0x18),-0x10);
    param_1[8] = uVar3;
    SetWindowLongA(*(HWND *)(param_1[1] + 0x18),-0x10,uVar3 & 0xfdffffff);
    hdc = GetDC(*(HWND *)(param_1[1] + 0x18));
    hPal = SelectPalette(hdc,DAT_006be5a0,0);
    pvVar5 = (void *)FUN_00618b70(0x28);
    uStack_4 = 0;
    if (pvVar5 == (void *)0x0) {
      pvVar5 = (void *)0x0;
    }
    else {
      pvVar5 = FUN_005fbea0(pvVar5,LVar1,LVar2,puVar8,hdc);
    }
    uStack_4 = 0xffffffff;
    param_1[2] = pvVar5;
    pvVar4 = (void *)0x0;
    if (pvVar5 != (void *)0x0) {
      pvVar5 = (void *)FUN_00618b70(0x28);
      uStack_4 = 1;
      if (pvVar5 == (void *)0x0) {
        pvVar5 = (void *)0x0;
      }
      else {
        pHVar9 = hdc;
        iVar6 = FUN_005fc0f0((int *)*param_1);
        iVar7 = FUN_005fc0e0((int *)*param_1);
        pvVar5 = FUN_005fbea0(pvVar5,iVar7,iVar6,puVar8,pHVar9);
      }
      uStack_4 = 0xffffffff;
      param_1[3] = pvVar5;
      pvVar4 = (void *)0x0;
      if (pvVar5 != (void *)0x0) {
        FUN_0060e120(param_1,aiStack_14,(int *)&stack0x00000004);
        FUN_0060df90(param_1,hdc,aiStack_14);
        SelectPalette(hdc,hPal,0);
        ReleaseDC(*(HWND *)(param_1[1] + 0x18),hdc);
        pvVar4 = (void *)0x1;
      }
    }
  }
  else {
    pvVar4 = (void *)0x0;
  }
  ExceptionList = local_c;
  return pvVar4;
}

