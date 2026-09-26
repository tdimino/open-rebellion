
void __thiscall FUN_005ff2d0(void *this,uint *param_1,int param_2,uint param_3)

{
  HWND hWnd;
  undefined4 *puVar1;
  HDC hDC;
  void *pvVar2;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_00655fa6;
  pvStack_c = ExceptionList;
  hWnd = *(HWND *)((int)this + 0x18);
  ExceptionList = &pvStack_c;
  hDC = GetDC(hWnd);
  if ((*(int *)((int)this + 0x94) == 0) && (*(int *)((int)this + 0x98) != 0)) {
    pvVar2 = (void *)FUN_00618b70(0x28);
    uStack_4 = 0;
    if (pvVar2 == (void *)0x0) {
LAB_005ff37b:
      pvVar2 = (void *)0x0;
    }
    else {
      pvVar2 = FUN_005fbea0(pvVar2,*(LONG *)((int)this + 0x38),*(LONG *)((int)this + 0x3c),
                            *(uint **)((int)this + 0x98),hDC);
    }
LAB_005ff37d:
    uStack_4 = 0xffffffff;
    *(void **)((int)this + 0x94) = pvVar2;
LAB_005ff38b:
    if (param_1 != (uint *)0x0) {
      if (*(void **)((int)this + 0x98) == (void *)0x0) {
        FUN_005fcc30(param_1,*(uint **)((int)this + 0x94),0,0,0,0,(undefined4 *)0x0,0);
      }
      else {
        if (*(uint **)((int)this + 0x94) == (uint *)0x0) goto LAB_005ff419;
        FUN_005fcc30(*(void **)((int)this + 0x98),*(uint **)((int)this + 0x94),0,0,param_2,param_3,
                     (undefined4 *)0x0,0);
        FUN_005fd0f0(param_1,*(uint **)((int)this + 0x94),0,0);
      }
      FUN_005fbfa0(param_1);
      FUN_00618b60((undefined *)param_1);
      goto LAB_005ff419;
    }
  }
  else if (param_1 != (uint *)0x0) {
    if (*(int *)((int)this + 0x94) == 0) {
      pvVar2 = (void *)FUN_00618b70(0x28);
      uStack_4 = 1;
      if (pvVar2 == (void *)0x0) goto LAB_005ff37b;
      pvVar2 = FUN_005fbea0(pvVar2,*(LONG *)((int)this + 0x38),*(LONG *)((int)this + 0x3c),param_1,
                            hDC);
      goto LAB_005ff37d;
    }
    goto LAB_005ff38b;
  }
  puVar1 = *(undefined4 **)((int)this + 0x94);
  if (puVar1 != (undefined4 *)0x0) {
    FUN_005fbfa0(puVar1);
    FUN_00618b60((undefined *)puVar1);
  }
  *(undefined4 *)((int)this + 0x94) = 0;
LAB_005ff419:
  ReleaseDC(hWnd,hDC);
  ExceptionList = pvStack_c;
  return;
}

