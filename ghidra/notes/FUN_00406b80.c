
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

HGDIOBJ FUN_00406b80(int *param_1,uint param_2,HWND param_3,int param_4)

{
  undefined1 *puVar1;
  double dVar2;
  HWND hWnd;
  float10 fVar3;
  bool bVar4;
  HDC pHVar5;
  int *piVar6;
  int iVar7;
  int iVar8;
  HGDIOBJ pvVar9;
  undefined4 extraout_ECX;
  HGDIOBJ unaff_EBP;
  bool bVar10;
  longlong lVar11;
  tagPAINTSTRUCT tStack_40;
  
  bVar10 = false;
  iVar7 = param_1[0x2a];
  hWnd = (HWND)param_1[6];
  if ((iVar7 < 0x14) || (0x17 < iVar7)) {
    bVar4 = false;
  }
  else {
    bVar4 = true;
  }
  if (0xf < param_2) {
    if (param_2 < 0x112) {
      if (param_2 == 0x111) {
        switch((uint)param_3 & 0xffff) {
        case 0x6f:
        case 0x70:
        case 0x71:
        case 0x72:
        case 0x73:
        case 0x74:
          puVar1 = (undefined1 *)((int)&param_3[-0x1c].unused + 2);
          bVar10 = false;
          param_1[0x29] = (int)puVar1;
          if ((param_1[0x27] == 4) || (param_1[0x27] == 5)) {
            iVar7 = 1;
          }
          else {
            iVar7 = 0;
          }
          iVar7 = FUN_00401910((int)puVar1,iVar7);
          if (iVar7 != 0) {
            bVar10 = param_1[(int)(&param_3[-0x1c].unused + 0xb)] == 0;
          }
          if (bVar10) {
            FUN_00416c50(0x478,(int)param_1,0x121e,0x121f);
            return (HGDIOBJ)0x0;
          }
switchD_00406fbb_caseD_478:
          FUN_00408000(param_1);
          return (HGDIOBJ)0x0;
        case 0x79:
        case 0x7a:
        case 0x7b:
        case 0x7c:
        case 0x7d:
        case 0x7e:
          piVar6 = param_1 + 0x2b;
          param_1[0x29] = (int)(param_3 + -0x1e);
          iVar8 = 0;
          do {
            if (*piVar6 == 1) {
              bVar10 = true;
              break;
            }
            iVar8 = iVar8 + 1;
            piVar6 = piVar6 + 1;
          } while (iVar8 < 6);
          if ((!bVar10) && (iVar7 != 0)) {
            FUN_00416c50(0x477,(int)param_1,0x121c,0x121d);
            return (HGDIOBJ)0x0;
          }
switchD_00406fbb_caseD_477:
          FUN_00407f60(param_1);
          return (HGDIOBJ)0x0;
        case 0x83:
        case 0x84:
        case 0x85:
        case 0x86:
        case 0x87:
        case 0x88:
          param_1[0x29] = (int)(param_3 + -0x1e);
          return (HGDIOBJ)0x0;
        case 0x8d:
          if (param_4 == 0) {
            iVar7 = -0x2580;
          }
          else {
            fVar3 = (float10)log2((float10)param_4 * (float10)_DAT_006585d8);
            dVar2 = (double)((float10)0.3010299956639812 * fVar3 * (float10)_DAT_006585e0);
            FUN_00619610(SUB84(dVar2,0),(uint)((ulonglong)dVar2 >> 0x20));
            lVar11 = __ftol();
            iVar7 = (int)lVar11;
          }
          if (bVar4) {
            FUN_00596b90();
          }
          FUN_00401aa0(iVar7);
          return (HGDIOBJ)0x0;
        case 0x8e:
          if (param_4 != 0) {
            fVar3 = (float10)log2((float10)param_4 * (float10)_DAT_006585d8);
            dVar2 = (double)((float10)0.3010299956639812 * fVar3 * (float10)_DAT_006585e0);
            FUN_00619610(SUB84(dVar2,0),(uint)((ulonglong)dVar2 >> 0x20));
            lVar11 = __ftol();
            FUN_00401a60((int)lVar11);
            return (HGDIOBJ)0x0;
          }
          FUN_00401a60(0xffffda80);
          return (HGDIOBJ)0x0;
        case 0x8f:
          FUN_00401040(0xd);
          return (HGDIOBJ)0x0;
        case 0x90:
          FUN_00401a20(param_1[0x28]);
          FUN_00401a40(param_1[0x28]);
          FUN_00604460(&DAT_006be4c8,8);
          if ((param_1[0x27] == 4) || (param_1[0x27] == 5)) {
            FUN_00597360();
            param_1[0x27] = 3;
          }
          FUN_00401040(param_1[0x27]);
          return (HGDIOBJ)0x0;
        case 0x91:
          FUN_00401a20(param_1[0x28]);
          FUN_00401a40(param_1[0x28]);
          if (param_1[0x2a] != 0) {
            bVar10 = false;
            iVar7 = 0;
            piVar6 = param_1 + 0x2b;
            do {
              if (*piVar6 == 1) {
                bVar10 = true;
                break;
              }
              iVar7 = iVar7 + 1;
              piVar6 = piVar6 + 1;
            } while (iVar7 < 6);
            if (!bVar10) {
              FUN_00408170(param_1,0x476);
              return (HGDIOBJ)0x0;
            }
          }
switchD_00406fbb_caseD_476:
          FUN_00401040(3);
          return (HGDIOBJ)0x0;
        case 0x99:
          FUN_004082b0(param_1,param_3);
          return (HGDIOBJ)0x0;
        case 0xa1:
        case 0xa2:
        case 0xa3:
        case 0xa4:
        case 0xa6:
          FUN_004082b0(param_1,param_3);
          return (HGDIOBJ)0x0;
        }
      }
      else {
        if (param_2 != 0x2b) goto LAB_00406f8a;
        if ((param_4 != 0) &&
           (piVar6 = (int *)FUN_00604500(param_1 + 0x1b,*(uint *)(param_4 + 4)),
           piVar6 != (int *)0x0)) {
          (**(code **)(*piVar6 + 0x24))
                    (*(undefined4 *)(param_4 + 0x18),
                     CONCAT31((int3)((uint)extraout_ECX >> 8),*(undefined1 *)(param_4 + 0x10)) &
                     0xffffff01);
          return unaff_EBP;
        }
      }
    }
    else if (param_2 == 0x405) {
      if (param_3 == (HWND)0xd) {
        thunk_FUN_00416d90();
      }
    }
    else {
      if (param_2 == 0x407) {
        PostMessageA(hWnd,0x111,(WPARAM)(param_3 + -5),0);
        return (HGDIOBJ)0x0;
      }
      if (param_2 != 0x46a) goto LAB_00406f8a;
      switch(param_3) {
      case (HWND)0x474:
        FUN_00408f10((int)param_1);
        InvalidateRect((HWND)param_1[6],(RECT *)0x0,0);
        return (HGDIOBJ)0x0;
      case (HWND)0x476:
        goto switchD_00406fbb_caseD_476;
      case (HWND)0x477:
        goto switchD_00406fbb_caseD_477;
      case (HWND)0x478:
        goto switchD_00406fbb_caseD_478;
      }
    }
    return (HGDIOBJ)0x0;
  }
  if (param_2 == 0xf) {
    pHVar5 = BeginPaint(hWnd,&tStack_40);
    (**(code **)(*param_1 + 0x28))();
    (**(code **)(*param_1 + 0x38))(pHVar5);
    EndPaint(hWnd,(PAINTSTRUCT *)&stack0xffffffb8);
    return unaff_EBP;
  }
  if (param_2 == 1) {
    FUN_0041d2f0(0);
    param_1[9] = 5;
    FUN_004070e0(param_1);
    FUN_004084b0(param_1);
    FUN_00407180(param_1);
    return (HGDIOBJ)0x0;
  }
LAB_00406f8a:
  pvVar9 = FUN_005ff530(param_1,param_2,param_3,param_4);
  return pvVar9;
}

