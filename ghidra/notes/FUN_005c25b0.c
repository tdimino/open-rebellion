
void __thiscall FUN_005c25b0(int param_1,undefined4 param_2)

{
  undefined4 uVar1;
  undefined4 uVar2;
  undefined4 uVar3;
  int *piVar4;
  int iVar5;
  HDC hDC;
  tagRECT tStack_38;
  undefined4 uStack_24;
  undefined1 auStack_20 [8];
  tagRECT tStack_18;
  undefined4 uStack_4;
  
  if (*(int *)(param_1 + 0x678) != 0) {
    return;
  }
  FUN_005fbbb0();
  *(undefined4 *)(param_1 + 0x658) = 0;
  *(undefined4 *)(param_1 + 0x638) = 0;
  if (*(int *)(param_1 + 0x644) == 0) {
    iVar5 = *(int *)(param_1 + 0x63c);
    if (iVar5 == 0) goto LAB_005c2631;
  }
  else {
    iVar5 = FUN_005d9ae0();
    if (iVar5 == 0) {
      *(undefined4 *)(param_1 + 0x638) = *(undefined4 *)(param_1 + 0x63c);
      if (*(undefined4 **)(param_1 + 0x644) != (undefined4 *)0x0) {
        (**(code **)**(undefined4 **)(param_1 + 0x644))(1);
      }
      *(undefined4 *)(param_1 + 0x644) = 0;
      goto LAB_005c2631;
    }
    iVar5 = *(int *)(param_1 + 0x644);
  }
  *(int *)(param_1 + 0x638) = iVar5;
LAB_005c2631:
  (**(code **)(**(int **)(param_1 + 0x638) + 0xc))();
  FUN_005c2c80();
  FUN_005c3080();
  iVar5 = FUN_005da080(0);
  if (iVar5 == 0) {
    return;
  }
  DAT_006bcbb4 = (**(code **)(**(int **)(param_1 + 0x138) + 0x94))
                           (*(int **)(param_1 + 0x138),param_2);
  if (DAT_006bcbb4 != 0) {
    FUN_005d93f0(0,DAT_006bcbb4,s_Moving_scene_failed__006abe64);
  }
  if (*(int *)(param_1 + 0x678) == 0) {
    if (*(int *)(param_1 + 0x144) == 0) {
      DAT_006bcbb4 = (**(code **)(**(int **)(param_1 + 0x130) + 0x38))(*(int **)(param_1 + 0x130));
      if (DAT_006bcbb4 != 0) {
        FUN_005d93f0(0,DAT_006bcbb4,s_Updating_device_failed__006abe28);
      }
      hDC = GetDC(*(HWND *)(param_1 + 0x18));
      iVar5 = FUN_00596a50();
      if (iVar5 == 0) {
        *(undefined4 *)(param_1 + 0x5c0) = 0;
        iVar5 = FUN_00596bc0();
        if ((iVar5 == 0) && (iVar5 = FUN_00595c60(), iVar5 == 1)) {
          *(undefined4 *)(param_1 + 0x5c4) = 1;
          FUN_00601ce0(hDC);
        }
        else {
          *(undefined4 *)(param_1 + 0x5c4) = 0;
        }
      }
      else {
        *(undefined4 *)(param_1 + 0x5c0) = 1;
        FUN_00601ce0(hDC);
      }
      iVar5 = FUN_00595c50();
      if (iVar5 == 1) {
        *(undefined4 *)(param_1 + 0x5c8) = 1;
        FUN_00403e90(2);
        FUN_00601ce0(hDC);
      }
      else {
        *(undefined4 *)(param_1 + 0x5c8) = 0;
      }
      ReleaseDC(*(HWND *)(param_1 + 0x18),hDC);
    }
    else {
      GetClientRect(*(HWND *)(param_1 + 0x18),&tStack_38);
      ClientToScreen(*(HWND *)(param_1 + 0x18),(LPPOINT)&stack0xffffffc0);
      SetRect(&tStack_18,tStack_38.left,tStack_38.top,tStack_38.right + tStack_38.left,
              tStack_38.bottom + tStack_38.top);
      uStack_4 = 0;
      iVar5 = (**(code **)(**(int **)(param_1 + 0x158) + 0x44))
                        (*(int **)(param_1 + 0x158),&uStack_4);
      if (iVar5 == 0) {
        iVar5 = FUN_00596a50();
        if (iVar5 == 1) {
          *(undefined4 *)(param_1 + 0x5c0) = 1;
          FUN_00601ce0(tStack_18.bottom);
        }
        else if (*(int *)(param_1 + 0x5c0) == 1) {
          iVar5 = *(int *)(param_1 + 0x62c);
          *(undefined4 *)(param_1 + 0x5c0) = 0;
          piVar4 = *(int **)(*(int *)(param_1 + 0x638) + 4);
          (**(code **)(*piVar4 + 0x5c))
                    (piVar4,*(undefined4 *)(iVar5 + 0x14),*(undefined4 *)(iVar5 + 0x18),
                     *(undefined4 *)(iVar5 + 0x1c),*(undefined4 *)(iVar5 + 0x20));
        }
        iVar5 = FUN_00596a50();
        if (iVar5 == 0) {
          iVar5 = FUN_00596bc0();
          if ((iVar5 == 0) && (iVar5 = FUN_00595c60(), iVar5 == 1)) {
            *(undefined4 *)(param_1 + 0x5c4) = 1;
            FUN_00601ce0(tStack_18.bottom);
          }
          else if (*(int *)(param_1 + 0x5c4) == 1) {
            iVar5 = *(int *)(param_1 + 0x630);
            *(undefined4 *)(param_1 + 0x5c4) = 0;
            piVar4 = *(int **)(*(int *)(param_1 + 0x638) + 4);
            (**(code **)(*piVar4 + 0x5c))
                      (piVar4,*(undefined4 *)(iVar5 + 0x14),*(undefined4 *)(iVar5 + 0x18),
                       *(undefined4 *)(iVar5 + 0x1c),*(undefined4 *)(iVar5 + 0x20));
          }
        }
        iVar5 = FUN_00595c50();
        if (iVar5 == 1) {
          *(undefined4 *)(param_1 + 0x5c8) = 1;
          FUN_00403e90(2);
          FUN_00601ce0(tStack_18.bottom);
        }
        else if (*(int *)(param_1 + 0x5c8) == 1) {
          iVar5 = *(int *)(param_1 + 0x634);
          *(undefined4 *)(param_1 + 0x5c8) = 0;
          uVar1 = *(undefined4 *)(iVar5 + 0x14);
          uVar2 = *(undefined4 *)(iVar5 + 0x18);
          uVar3 = *(undefined4 *)(iVar5 + 0x1c);
          uStack_24 = *(undefined4 *)(iVar5 + 0x20);
          FUN_00403e90(2);
          piVar4 = *(int **)(*(int *)(param_1 + 0x638) + 4);
          (**(code **)(*piVar4 + 0x5c))(piVar4,uVar1,uVar2,uVar3,uStack_24);
        }
        (**(code **)(**(int **)(param_1 + 0x158) + 0x68))
                  (*(int **)(param_1 + 0x158),tStack_18.bottom);
      }
      DAT_006bcbb4 = (**(code **)(**(int **)(param_1 + 0x154) + 0x14))
                               (*(int **)(param_1 + 0x154),auStack_20,
                                *(undefined4 *)(param_1 + 0x158),&stack0xffffffc0,0x1000000,0);
      if (DAT_006bcbb4 != 0) {
        FUN_005d93f0(0,DAT_006bcbb4,s_Blt_of_back_to_front_buffer_fail_006abe40);
        FUN_005c2c20();
        return;
      }
    }
  }
  FUN_005c2c20();
  return;
}

