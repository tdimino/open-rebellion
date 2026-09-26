
int __thiscall FUN_00428b40(void *this,int *param_1)

{
  uint uVar1;
  int iVar2;
  int *piVar3;
  undefined4 *puVar4;
  BOOL BVar5;
  int iVar6;
  int *piVar7;
  int *piVar8;
  tagRECT tStack_44;
  tagRECT local_34;
  undefined1 local_24 [4];
  uint local_20;
  undefined4 local_1c;
  uint local_14;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  piVar3 = param_1;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062c2d8;
  pvStack_c = ExceptionList;
  piVar7 = (int *)0x0;
  ExceptionList = &pvStack_c;
  FUN_00610b20(local_24,0,0,0,0,0,0);
  local_14 = local_14 & 0xfffffff9 | 1;
  local_4 = 0;
  local_20 = 0x263 - (*(int *)((int)this + 0x9c) != 1) & 0xffff;
  local_1c = 7;
  SetRect(&local_34,param_1[10],param_1[0xb],param_1[0xe] + param_1[10],param_1[0xf] + param_1[0xb])
  ;
  piVar8 = (int *)((int)this + 0x2f0);
  do {
    if (*piVar8 == 0) {
      CopyRect(&tStack_44,(RECT *)(piVar8 + -4));
      iVar6 = param_1[9];
      piVar8[-5] = 0;
      *piVar8 = iVar6;
      *(int *)((int)this + *(int *)((int)this + 0x458) * 4 + 0x428) = param_1[9];
      *(int *)((int)this + 0x458) = *(int *)((int)this + 0x458) + 1;
      param_1 = piVar7;
      break;
    }
    if (*piVar8 == param_1[9]) {
      CopyRect(&tStack_44,(RECT *)((int)this + (int)piVar7 * 0x1c + 0x2e0));
      param_1 = piVar7;
      break;
    }
    piVar7 = (int *)((int)piVar7 + 1);
    piVar8 = piVar8 + 7;
  } while ((int)piVar7 < 0xc);
  if (piVar7 == (int *)0xc) {
    uVar1 = *(uint *)((int)this + 0x428);
    puVar4 = (undefined4 *)((int)this + 0x428);
    iVar6 = 0;
    if (0 < *(int *)((int)this + 0x458) + -1) {
      do {
        iVar6 = iVar6 + 1;
        *puVar4 = puVar4[1];
        puVar4 = puVar4 + 1;
      } while (iVar6 < *(int *)((int)this + 0x458) + -1);
    }
    iVar6 = (int)this + 0x2d8;
    piVar8 = (int *)0x0;
    *(int *)((int)this + 0x458) = *(int *)((int)this + 0x458) + -1;
    do {
      if (*(uint *)(iVar6 + 0x18) == uVar1) {
        FUN_005f54a0((void *)((int)this + 0x478),uVar1);
        FUN_00600f90(this,uVar1);
        CopyRect(&tStack_44,(RECT *)((int)this + (int)piVar8 * 0x1c + 0x2e0));
        iVar2 = piVar3[9];
        *(undefined4 *)(iVar6 + 4) = 0;
        *(int *)(iVar6 + 0x18) = iVar2;
        *(int *)((int)this + *(int *)((int)this + 0x458) * 4 + 0x428) = piVar3[9];
        *(int *)((int)this + 0x458) = *(int *)((int)this + 0x458) + 1;
        param_1 = piVar8;
        break;
      }
      piVar8 = (int *)((int)piVar8 + 1);
      iVar6 = iVar6 + 0x1c;
    } while ((int)piVar8 < 0xc);
  }
  if ((*(byte *)((int)this + 0xc0) & 4) == 0) {
    FUN_00610c30((int)local_24);
    FUN_00428940(*(HWND *)((int)this + 0x18),&tStack_44.left,&local_34.left);
  }
  BVar5 = IsWindowVisible((HWND)piVar3[6]);
  if ((BVar5 != 0) && ((*(byte *)((int)this + 0xc0) & 4) == 0)) {
    (**(code **)(*piVar3 + 8))();
  }
  FUN_00428d70(this,piVar3);
  ExceptionList = pvStack_c;
  return (int)param_1 + 1;
}

