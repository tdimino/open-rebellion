
void __thiscall FUN_00414330(void *this,int param_1)

{
  undefined4 uVar1;
  uint uVar2;
  int iVar3;
  BOOL BVar4;
  
  *(undefined4 *)((int)this + 0xe0) = *(undefined4 *)((int)this + 0xdc);
  *(int *)((int)this + 0xdc) = param_1;
  uVar2 = param_1 - 1;
  if (uVar2 < 0x13) {
    do {
      switch((&switchD_00414365::switchdataD_00414694)
             [(&switchD_00414365::switchdataD_004146c4)[uVar2]]) {
      case (undefined *)0x41436c:
        iVar3 = FUN_00401c10();
        if (iVar3 == 7) {
          if (*(int **)((int)this + 0xc0) != (int *)0x0) {
            (**(code **)(**(int **)((int)this + 0xc0) + 8))();
            iVar3 = *(int *)((int)this + 0xc0);
            if (*(int *)((int)this + 0xb8) == iVar3) {
              *(undefined4 *)((int)this + 0xb8) = 0;
            }
            if (*(int *)((int)this + 0xbc) == iVar3) {
              *(undefined4 *)((int)this + 0xbc) = 0;
            }
            PostMessageA(*(HWND *)((int)this + 0x18),0x469,0,iVar3);
            *(undefined4 *)((int)this + 0xc0) = 0;
          }
          FUN_004150d0((int)this);
          FUN_00415300((int)this);
          iVar3 = 9;
        }
        else {
          if (iVar3 != 0x16) {
            return;
          }
          if (*(int **)((int)this + 0xc0) != (int *)0x0) {
            (**(code **)(**(int **)((int)this + 0xc0) + 8))();
            iVar3 = *(int *)((int)this + 0xc0);
            if (*(int *)((int)this + 0xb8) == iVar3) {
              *(undefined4 *)((int)this + 0xb8) = 0;
            }
            if (*(int *)((int)this + 0xbc) == iVar3) {
              *(undefined4 *)((int)this + 0xbc) = 0;
            }
            PostMessageA(*(HWND *)((int)this + 0x18),0x469,0,iVar3);
            *(undefined4 *)((int)this + 0xc0) = 0;
          }
          FUN_004150d0((int)this);
          FUN_00415300((int)this);
          iVar3 = 8;
        }
        uVar1 = *(undefined4 *)((int)this + 0xdc);
        *(int *)((int)this + 0xdc) = iVar3;
        uVar2 = iVar3 - 1;
        *(undefined4 *)((int)this + 0xe0) = uVar1;
        if (0x12 < uVar2) {
          *(undefined4 *)((int)this + 0xdc) = 0;
          return;
        }
        break;
      case (undefined *)0x41445b:
        *(undefined4 *)((int)this + 0xfc) = 0;
        *(undefined4 *)((int)this + 0x100) = 0;
        *(undefined4 *)((int)this + 0xf8) = 0;
        FUN_004163d0((int)this);
        FUN_00414830(this);
        return;
      case (undefined *)0x414481:
        *(undefined4 *)((int)this + 0xfc) = 0;
        *(undefined4 *)((int)this + 0x100) = 0;
        *(undefined4 *)((int)this + 0xf8) = 0;
        *(undefined4 *)((int)this + 0x104) = 0;
        *(undefined4 *)((int)this + 0xe4) = 0;
        FUN_005974c0();
        FUN_00597050();
      case (undefined *)0x4144a9:
        FUN_004163d0((int)this);
        FUN_00414a80(this,*(undefined4 *)((int)this + 0xe0));
        return;
      case (undefined *)0x4144c4:
        FUN_004163d0((int)this);
        FUN_00415120((int)this);
        return;
      case (undefined *)0x4144d8:
        *(undefined4 *)((int)this + 0xfc) = 1;
        FUN_004163d0((int)this);
        FUN_00414eb0((int)this);
        return;
      case (undefined *)0x4144f8:
        FUN_004163d0((int)this);
        thunk_FUN_00416d90();
        iVar3 = *(int *)((int)this + 0xb8);
        if (iVar3 != 0) {
          if (*(int *)((int)this + 0xc0) == iVar3) {
            *(undefined4 *)((int)this + 0xc0) = 0;
          }
          BVar4 = DestroyWindow(*(HWND *)(iVar3 + 0x18));
          if ((BVar4 != 0) && (*(undefined4 **)((int)this + 0xb8) != (undefined4 *)0x0)) {
            (**(code **)**(undefined4 **)((int)this + 0xb8))(1);
          }
          *(undefined4 *)((int)this + 0xb8) = 0;
        }
        iVar3 = *(int *)((int)this + 0xbc);
        if (iVar3 != 0) {
          if (*(int *)((int)this + 0xc0) == iVar3) {
            *(undefined4 *)((int)this + 0xc0) = 0;
          }
          DestroyWindow(*(HWND *)(iVar3 + 0x18));
          if (*(undefined4 **)((int)this + 0xbc) != (undefined4 *)0x0) {
            (**(code **)**(undefined4 **)((int)this + 0xbc))(1);
          }
          *(undefined4 *)((int)this + 0xbc) = 0;
        }
        FUN_00597050();
        thunk_FUN_00409370();
        FUN_00414830(this);
        return;
      case (undefined *)0x41458d:
        FUN_00601080((HWND)0x0);
        SetFocus((HWND)0x0);
        if (*(int **)((int)this + 0xc4) != (int *)0x0) {
          (**(code **)(**(int **)((int)this + 0xc4) + 8))();
          DestroyWindow(*(HWND *)(*(int *)((int)this + 0xc4) + 0x18));
          if (*(undefined4 **)((int)this + 0xc4) != (undefined4 *)0x0) {
            (**(code **)**(undefined4 **)((int)this + 0xc4))(1);
          }
          *(undefined4 *)((int)this + 0xc4) = 0;
        }
        FUN_00401110(0x470,0,0x1202);
        *(undefined4 *)((int)this + 0xdc) = *(undefined4 *)((int)this + 0xe0);
        return;
      case (undefined *)0x4145fd:
        WinHelpA(*(HWND *)((int)this + 0x18),(LPCSTR)0x0,2,0);
        FUN_00597340();
        FUN_00597360();
        thunk_FUN_005975a0();
        if (*(int **)((int)this + 0xc4) != (int *)0x0) {
          (**(code **)(**(int **)((int)this + 0xc4) + 8))();
          DestroyWindow(*(HWND *)(*(int *)((int)this + 0xc4) + 0x18));
          if (*(undefined4 **)((int)this + 0xc4) != (undefined4 *)0x0) {
            (**(code **)**(undefined4 **)((int)this + 0xc4))(1);
          }
          *(undefined4 *)((int)this + 0xc4) = 0;
        }
        FUN_00415350((int)this);
        FUN_00597030();
        thunk_FUN_00409370();
        FUN_00610c00(0);
        FUN_00614290();
        PostQuitMessage(0);
        return;
      case (undefined *)0x414688:
        goto switchD_00414365_caseD_414688;
      }
    } while( true );
  }
switchD_00414365_caseD_414688:
  *(undefined4 *)((int)this + 0xdc) = 0;
  return;
}

