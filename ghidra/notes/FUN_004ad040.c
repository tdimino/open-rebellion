
void __thiscall FUN_004ad040(void *this,uint *param_1)

{
  undefined4 *puVar1;
  bool bVar2;
  HDC hDC;
  undefined3 extraout_var;
  int nWidth;
  int iVar3;
  void *pvVar4;
  undefined3 extraout_var_00;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_00638776;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  hDC = GetDC(*(HWND *)((int)this + 0x18));
  puVar1 = *(undefined4 **)((int)this + 0xb8);
  if (((puVar1 != (undefined4 *)0x0) && (param_1 != (uint *)0x0)) && (puVar1 != (undefined4 *)0x0))
  {
    FUN_005fbfa0(puVar1);
    FUN_00618b60((undefined *)puVar1);
  }
  bVar2 = FUN_005fc040((int *)param_1);
  if ((CONCAT31(extraout_var,bVar2) != 0) && (*(int *)((int)this + 0xb8) == 0)) {
    puVar1 = *(undefined4 **)((int)this + 0xbc);
    if (puVar1 != (undefined4 *)0x0) {
      FUN_005fbfa0(puVar1);
      FUN_00618b60((undefined *)puVar1);
    }
    nWidth = FUN_005fc0e0((int *)param_1);
    iVar3 = FUN_005fc0f0((int *)param_1);
    pvVar4 = (void *)FUN_00618b70(0x28);
    uStack_4 = 0;
    if (pvVar4 == (void *)0x0) {
      pvVar4 = (void *)0x0;
    }
    else {
      pvVar4 = FUN_005fbda0(pvVar4,nWidth,iVar3,param_1);
    }
    uStack_4 = 0xffffffff;
    *(void **)((int)this + 0xb8) = pvVar4;
    pvVar4 = (void *)FUN_00618b70(0x28);
    uStack_4 = 1;
    if (pvVar4 == (void *)0x0) {
      pvVar4 = (void *)0x0;
    }
    else {
      pvVar4 = FUN_005fbea0(pvVar4,nWidth,iVar3,param_1,hDC);
    }
    *(void **)((int)this + 0xbc) = pvVar4;
    uStack_4 = 0xffffffff;
    MoveWindow(*(HWND *)((int)this + 0x18),*(int *)((int)this + 0x28),*(int *)((int)this + 0x2c),
               nWidth,iVar3 + -1,0);
  }
  bVar2 = FUN_005fc040((int *)param_1);
  if (CONCAT31(extraout_var_00,bVar2) == 0) {
    puVar1 = *(undefined4 **)((int)this + 0xb8);
    if (puVar1 != (undefined4 *)0x0) {
      FUN_005fbfa0(puVar1);
      FUN_00618b60((undefined *)puVar1);
    }
    puVar1 = *(undefined4 **)((int)this + 0xbc);
    if (puVar1 != (undefined4 *)0x0) {
      FUN_005fbfa0(puVar1);
      FUN_00618b60((undefined *)puVar1);
    }
    *(undefined4 *)((int)this + 0xb8) = 0;
    *(undefined4 *)((int)this + 0xbc) = 0;
  }
  else {
    FUN_005fcc30(param_1,*(uint **)((int)this + 0xb8),0,0,0,0,(undefined4 *)0x0,0);
    if (param_1 != (uint *)0x0) {
      FUN_005fbfa0(param_1);
      FUN_00618b60((undefined *)param_1);
    }
  }
  ReleaseDC(*(HWND *)((int)this + 0x18),hDC);
  ExceptionList = pvStack_c;
  return;
}

