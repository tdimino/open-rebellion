
WPARAM __thiscall FUN_00413560(void *this,undefined4 param_1)

{
  bool bVar1;
  bool bVar2;
  HWND pHVar3;
  int iVar4;
  undefined4 extraout_ECX;
  int unaff_EBX;
  undefined4 uVar5;
  undefined4 uVar6;
  undefined4 uVar7;
  uint local_24;
  tagMSG local_20;
  
  local_24 = 0;
  local_20.hwnd = (HWND)0x1;
  FUN_00597460();
  FUN_005ffce0(this,0);
  (**(code **)(*(int *)this + 4))(param_1);
  do {
    iVar4 = PeekMessageA(&local_20,(HWND)0x0,0,0,1);
    while ((iVar4 != 0 && (unaff_EBX == 0))) {
      if (local_20.message == 0x12) {
        unaff_EBX = 1;
      }
      iVar4 = (**(code **)(*(int *)this + 0x24))(&local_20);
      pHVar3 = local_20.hwnd;
      if (iVar4 != 0) {
        if ((*(HWND *)((int)this + 0x18) != (HWND)0x0) && (*(int *)((int)this + 0x98) != 0)) {
          local_20.hwnd = *(HWND *)((int)this + 0x18);
        }
        iVar4 = TranslateAcceleratorA(local_20.hwnd,*(HACCEL *)((int)this + 0x98),&local_20);
        if (iVar4 == 0) {
          local_20.hwnd = pHVar3;
          TranslateMessage(&local_20);
          DispatchMessageA(&local_20);
        }
      }
      iVar4 = PeekMessageA(&local_20,(HWND)0x0,0,0,1);
    }
    local_24 = 1;
    FUN_00401bd0(&local_24);
    if ((*(int **)((int)this + 0xc0) != (int *)0x0) &&
       (*(int **)((int)this + 0xc0) == *(int **)((int)this + 0xb8))) {
      (**(code **)(**(int **)((int)this + 0xb8) + 0x2c))();
    }
    if ((*(int *)((int)this + 0x104) == 7) && (*(int *)((int)this + 0xe4) == 2)) {
      FUN_00597310();
      iVar4 = FUN_00597090();
      *(int *)((int)this + 0xf4) = iVar4;
      if (*(int *)((int)this + 0xfc) != 0) {
        if (iVar4 == 2) {
          *(undefined4 *)((int)this + 0x100) = 0;
          *(undefined4 *)((int)this + 0xf8) = 0;
        }
        else {
          *(undefined4 *)((int)this + 0xf8) = 1;
        }
        if ((((*(int *)((int)this + 0xf8) != 0) && (*(int *)((int)this + 0x100) == 0)) &&
            (unaff_EBX == 0)) &&
           ((7 < *(uint *)((int)this + 0xdc) && (*(uint *)((int)this + 0xdc) < 0x10)))) {
          *(undefined4 *)((int)this + 0x100) = 1;
          FUN_00597050();
          iVar4 = *(int *)((int)this + 0xdc);
          if (iVar4 == 8) {
            if (*(int *)(*(int *)((int)this + 0xb8) + 0xac) != 0) goto LAB_00413736;
          }
          else if ((iVar4 == 4) || (iVar4 == 5)) {
            FUN_004109e0(*(int *)((int)this + 0xc0));
          }
          FUN_00401150(0x46d,0,0x1204,0x190e);
          FUN_00597050();
        }
      }
    }
LAB_00413736:
    if (local_24 != 0) {
      bVar1 = false;
      bVar2 = false;
      iVar4 = FUN_004fd160();
      if ((iVar4 != 0) && (iVar4 = FUN_004fd180(), iVar4 == 0)) {
        bVar1 = true;
        bVar2 = true;
      }
      if (bVar1) {
LAB_0041379f:
        FUN_004fd140(1);
        FUN_004fd1a0(1);
        FUN_004fd200(1);
        FUN_0041dbc0(extraout_ECX);
        iVar4 = *(int *)((int)this + 0xdc);
        if (bVar2) {
          if ((iVar4 == 4) || (iVar4 == 5)) {
            FUN_004109e0(*(int *)((int)this + 0xc0));
          }
          if (*(int *)((int)this + 0x154) == 0) {
            uVar7 = 0x1911;
            uVar6 = 0x120b;
            uVar5 = 0x47d;
LAB_0041386c:
            FUN_00401150(uVar5,uVar6,0,uVar7);
          }
          else {
            FUN_00401110(0x47d,0x120b,0x120c);
          }
        }
        else {
          if ((iVar4 == 4) || (iVar4 == 5)) {
            FUN_004109e0(*(int *)((int)this + 0xc0));
          }
          if (*(int *)((int)this + 0x154) == 0) {
            uVar7 = 0x1910;
            uVar6 = 0x1209;
            uVar5 = 0x47c;
            goto LAB_0041386c;
          }
          FUN_00401110(0x47c,0x1209,0x120a);
        }
      }
      else {
        iVar4 = FUN_004fd1c0();
        bVar1 = false;
        if ((iVar4 != 0) && (iVar4 = FUN_004fd1e0(), iVar4 == 0)) {
          bVar1 = true;
          bVar2 = false;
        }
        if (bVar1) goto LAB_0041379f;
        iVar4 = FUN_004fd100();
        bVar1 = false;
        if ((iVar4 != 0) && (iVar4 = FUN_004fd120(), iVar4 == 0)) {
          bVar1 = true;
          bVar2 = false;
        }
        if (bVar1) goto LAB_0041379f;
      }
      FUN_004161b0(this);
    }
    if (unaff_EBX != 0) {
      return local_20.wParam;
    }
  } while( true );
}

