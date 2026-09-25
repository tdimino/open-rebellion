
void __fastcall FUN_00456230(void *param_1)

{
  undefined4 *puVar1;
  uint uVar2;
  HDC hDC;
  void *pvVar3;
  int *this;
  int iVar4;
  undefined4 *puVar5;
  uint local_18;
  uint uStack_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_006302b7;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  uVar2 = FUN_006037f0(7);
  hDC = GetDC(*(HWND *)((int)param_1 + 0x18));
  puVar5 = *(undefined4 **)((int)param_1 + 0x180);
  if (puVar5 != (undefined4 *)0x0) {
    FUN_005fbfa0(puVar5);
    FUN_00618b60((undefined *)puVar5);
  }
  puVar5 = *(undefined4 **)((int)param_1 + 0x184);
  if (puVar5 != (undefined4 *)0x0) {
    FUN_005fbfa0(puVar5);
    FUN_00618b60((undefined *)puVar5);
  }
  puVar5 = *(undefined4 **)((int)param_1 + 0x15c);
  if (puVar5 != (undefined4 *)0x0) {
    FUN_005fbfa0(puVar5);
    FUN_00618b60((undefined *)puVar5);
  }
  puVar5 = *(undefined4 **)((int)param_1 + 0x18c);
  if (puVar5 != (undefined4 *)0x0) {
    FUN_005fbfa0(puVar5);
    FUN_00618b60((undefined *)puVar5);
  }
  puVar5 = *(undefined4 **)((int)param_1 + 400);
  if (puVar5 != (undefined4 *)0x0) {
    FUN_005fbfa0(puVar5);
    FUN_00618b60((undefined *)puVar5);
  }
  puVar5 = (undefined4 *)((int)param_1 + 0x254);
  iVar4 = 3;
  do {
    puVar1 = (undefined4 *)*puVar5;
    if (puVar1 != (undefined4 *)0x0) {
      FUN_005fbfa0(puVar1);
      FUN_00618b60((undefined *)puVar1);
    }
    *puVar5 = 0;
    puVar5 = puVar5 + 1;
    iVar4 = iVar4 + -1;
  } while (iVar4 != 0);
  iVar4 = *(int *)((int)param_1 + 0x148);
  if (iVar4 == 1) {
    pvVar3 = (void *)FUN_00618b70(0x28);
    uStack_4 = 0;
    if (pvVar3 == (void *)0x0) {
      pvVar3 = (void *)0x0;
    }
    else {
      pvVar3 = FUN_005fbd20(pvVar3,uVar2,0x283b,10);
    }
    uStack_4 = 0xffffffff;
    *(void **)((int)param_1 + 0x180) = pvVar3;
    pvVar3 = (void *)FUN_00618b70(0x28);
    uStack_4 = 1;
    if (pvVar3 == (void *)0x0) {
      pvVar3 = (void *)0x0;
    }
    else {
      pvVar3 = FUN_005fbd20(pvVar3,uVar2,0x27d8,10);
    }
    uStack_4 = 0xffffffff;
    *(void **)((int)param_1 + 0x184) = pvVar3;
    pvVar3 = (void *)FUN_00618b70(0x28);
    uStack_4 = 2;
    if (pvVar3 == (void *)0x0) {
      pvVar3 = (void *)0x0;
    }
    else {
      pvVar3 = FUN_005fbd20(pvVar3,uVar2,0x2816,10);
    }
    uStack_4 = 0xffffffff;
    *(void **)((int)param_1 + 0x15c) = pvVar3;
    pvVar3 = (void *)FUN_00618b70(0x28);
    uStack_4 = 3;
    if (pvVar3 == (void *)0x0) {
      pvVar3 = (void *)0x0;
    }
    else {
      pvVar3 = FUN_005fbd20(pvVar3,uVar2,0x2833,10);
    }
    uStack_4 = 0xffffffff;
    *(void **)((int)param_1 + 0x18c) = pvVar3;
    pvVar3 = (void *)FUN_00618b70(0x28);
    uStack_4 = 4;
    if (pvVar3 == (void *)0x0) {
      pvVar3 = (void *)0x0;
    }
    else {
      pvVar3 = FUN_005fbd20(pvVar3,uVar2,0x2834,10);
    }
    *(void **)((int)param_1 + 400) = pvVar3;
    uStack_4 = 0xffffffff;
    local_18 = 0x2848;
    uStack_10 = 0x2847;
    if (*(int *)(*(int *)((int)param_1 + 0x14c) + 0x9c) != 1) goto LAB_00456843;
    pvVar3 = (void *)FUN_00618b70(0x28);
    uStack_4 = 5;
    if (pvVar3 == (void *)0x0) {
      pvVar3 = (void *)0x0;
    }
    else {
      pvVar3 = FUN_005fbd20(pvVar3,uVar2,0x2841,10);
    }
    uStack_4 = 0xffffffff;
    *(void **)((int)param_1 + 0x254) = pvVar3;
    pvVar3 = (void *)FUN_00618b70(0x28);
    uStack_4 = 6;
    if (pvVar3 == (void *)0x0) {
      pvVar3 = (void *)0x0;
    }
    else {
      pvVar3 = FUN_005fbd20(pvVar3,uVar2,0x2842,10);
    }
    uStack_4 = 0xffffffff;
    *(void **)((int)param_1 + 600) = pvVar3;
    pvVar3 = (void *)FUN_00618b70(0x28);
    uStack_4 = 7;
    if (pvVar3 != (void *)0x0) {
      pvVar3 = FUN_005fbd20(pvVar3,uVar2,0x2843,10);
      uStack_4 = 0xffffffff;
      *(void **)((int)param_1 + 0x25c) = pvVar3;
      goto LAB_00456843;
    }
  }
  else {
    if (iVar4 != 2) {
      local_18 = uVar2;
      if (iVar4 == 3) {
        pvVar3 = (void *)FUN_00618b70(0x28);
        uStack_4 = 0x10;
        if (pvVar3 == (void *)0x0) {
          pvVar3 = (void *)0x0;
        }
        else {
          pvVar3 = FUN_005fbd20(pvVar3,uVar2,0x283f,10);
        }
        uStack_4 = 0xffffffff;
        *(void **)((int)param_1 + 0x180) = pvVar3;
        pvVar3 = (void *)FUN_00618b70(0x28);
        uStack_4 = 0x11;
        if (pvVar3 == (void *)0x0) {
          pvVar3 = (void *)0x0;
        }
        else {
          pvVar3 = FUN_005fbd20(pvVar3,uVar2,0x2840,10);
        }
        uStack_4 = 0xffffffff;
        *(void **)((int)param_1 + 0x184) = pvVar3;
        pvVar3 = (void *)FUN_00618b70(0x28);
        uStack_4 = 0x12;
        if (pvVar3 == (void *)0x0) {
          pvVar3 = (void *)0x0;
        }
        else {
          pvVar3 = FUN_005fbd20(pvVar3,uVar2,0x2818,10);
        }
        uStack_4 = 0xffffffff;
        *(void **)((int)param_1 + 0x15c) = pvVar3;
        pvVar3 = (void *)FUN_00618b70(0x28);
        uStack_4 = 0x13;
        if (pvVar3 == (void *)0x0) {
          pvVar3 = (void *)0x0;
        }
        else {
          pvVar3 = FUN_005fbd20(pvVar3,uVar2,0x2837,10);
        }
        uStack_4 = 0xffffffff;
        *(void **)((int)param_1 + 0x18c) = pvVar3;
        pvVar3 = (void *)FUN_00618b70(0x28);
        uStack_4 = 0x14;
        if (pvVar3 == (void *)0x0) {
          pvVar3 = (void *)0x0;
        }
        else {
          pvVar3 = FUN_005fbd20(pvVar3,uVar2,0x2838,10);
        }
        uStack_4 = 0xffffffff;
        *(void **)((int)param_1 + 400) = pvVar3;
        local_18 = 0x284e;
        uStack_10 = 0x284d;
      }
      goto LAB_00456843;
    }
    pvVar3 = (void *)FUN_00618b70(0x28);
    uStack_4 = 8;
    if (pvVar3 == (void *)0x0) {
      pvVar3 = (void *)0x0;
    }
    else {
      pvVar3 = FUN_005fbd20(pvVar3,uVar2,0x27d9,10);
    }
    uStack_4 = 0xffffffff;
    *(void **)((int)param_1 + 0x180) = pvVar3;
    pvVar3 = (void *)FUN_00618b70(0x28);
    uStack_4 = 9;
    if (pvVar3 == (void *)0x0) {
      pvVar3 = (void *)0x0;
    }
    else {
      pvVar3 = FUN_005fbd20(pvVar3,uVar2,0x283e,10);
    }
    uStack_4 = 0xffffffff;
    *(void **)((int)param_1 + 0x184) = pvVar3;
    pvVar3 = (void *)FUN_00618b70(0x28);
    uStack_4 = 10;
    if (pvVar3 == (void *)0x0) {
      pvVar3 = (void *)0x0;
    }
    else {
      pvVar3 = FUN_005fbd20(pvVar3,uVar2,0x2817,10);
    }
    uStack_4 = 0xffffffff;
    *(void **)((int)param_1 + 0x15c) = pvVar3;
    pvVar3 = (void *)FUN_00618b70(0x28);
    uStack_4 = 0xb;
    if (pvVar3 == (void *)0x0) {
      pvVar3 = (void *)0x0;
    }
    else {
      pvVar3 = FUN_005fbd20(pvVar3,uVar2,0x2835,10);
    }
    uStack_4 = 0xffffffff;
    *(void **)((int)param_1 + 0x18c) = pvVar3;
    pvVar3 = (void *)FUN_00618b70(0x28);
    uStack_4 = 0xc;
    if (pvVar3 == (void *)0x0) {
      pvVar3 = (void *)0x0;
    }
    else {
      pvVar3 = FUN_005fbd20(pvVar3,uVar2,0x2836,10);
    }
    *(void **)((int)param_1 + 400) = pvVar3;
    uStack_4 = 0xffffffff;
    local_18 = 0x284b;
    uStack_10 = 0x284a;
    if (*(int *)(*(int *)((int)param_1 + 0x14c) + 0x9c) != 2) goto LAB_00456843;
    pvVar3 = (void *)FUN_00618b70(0x28);
    uStack_4 = 0xd;
    if (pvVar3 == (void *)0x0) {
      pvVar3 = (void *)0x0;
    }
    else {
      pvVar3 = FUN_005fbd20(pvVar3,uVar2,0x2844,10);
    }
    uStack_4 = 0xffffffff;
    *(void **)((int)param_1 + 0x254) = pvVar3;
    pvVar3 = (void *)FUN_00618b70(0x28);
    uStack_4 = 0xe;
    if (pvVar3 == (void *)0x0) {
      pvVar3 = (void *)0x0;
    }
    else {
      pvVar3 = FUN_005fbd20(pvVar3,uVar2,0x2845,10);
    }
    uStack_4 = 0xffffffff;
    *(void **)((int)param_1 + 600) = pvVar3;
    pvVar3 = (void *)FUN_00618b70(0x28);
    uStack_4 = 0xf;
    if (pvVar3 != (void *)0x0) {
      pvVar3 = FUN_005fbd20(pvVar3,uVar2,0x2846,10);
      uStack_4 = 0xffffffff;
      *(void **)((int)param_1 + 0x25c) = pvVar3;
      goto LAB_00456843;
    }
  }
  uStack_4 = 0xffffffff;
  *(undefined4 *)((int)param_1 + 0x25c) = 0;
LAB_00456843:
  this = (int *)FUN_00607e90(param_1,0x67);
  if (this != (int *)0x0) {
    FUN_00603150(this,0,local_18);
    FUN_00603150(this,1,uStack_10);
    (**(code **)(*this + 4))(5);
  }
  ReleaseDC(*(HWND *)((int)param_1 + 0x18),hDC);
  ExceptionList = pvStack_c;
  return;
}

