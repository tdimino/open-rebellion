
uint __thiscall FUN_00618510(void *this,int param_1)

{
  uint *puVar1;
  char cVar2;
  int *piVar3;
  int iVar4;
  uint uVar5;
  uint uVar6;
  char *pcVar7;
  uint uVar8;
  int iVar9;
  int local_8;
  int local_4;
  
  iVar9 = 0;
  local_8 = 0;
  local_4 = 1;
  uVar8 = *(uint *)(*(int *)(*(int *)this + 4) + 0x24 + (int)this);
  if ((uVar8 & 0x10) == 0) {
    if ((uVar8 & 0x40) == 0) {
      uVar8 = ((int)(char)uVar8 & 0x20U) >> 2;
    }
    else {
      uVar8 = 0x10;
    }
  }
  else {
    uVar8 = 10;
  }
  iVar4 = FUN_00617a90(this,0);
  if (iVar4 != 0) {
    uVar5 = FUN_006184e0(*(int **)(*(int *)(*(int *)this + 4) + 4 + (int)this));
    while (uVar5 != 0xffffffff) {
      if ((iVar9 == 0) && ((uVar5 == 0x2d || (uVar5 == 0x2b)))) {
        local_4 = local_4 + 1;
      }
      else {
        if ((iVar9 == local_4) && (*(char *)(iVar9 + -1 + param_1) == '0')) {
          if (((uVar5 == 0x78) || (uVar5 == 0x58)) && ((uVar8 == 0 || (uVar8 == 0x10)))) {
            uVar8 = 0x10;
            local_8 = 0;
            goto LAB_006185ef;
          }
          if (uVar8 == 0) {
            uVar8 = 8;
          }
        }
        if (uVar8 == 0x10) {
          uVar6 = FUN_00619fc0(uVar5);
          if (uVar6 == 0) goto LAB_00618624;
        }
        else {
          uVar6 = FUN_00619f90(uVar5);
          if ((uVar6 == 0) || ((uVar8 == 8 && (0x37 < (int)uVar5)))) goto LAB_00618624;
        }
        local_8 = local_8 + 1;
      }
LAB_006185ef:
      *(char *)(iVar9 + param_1) = (char)uVar5;
      uVar5 = FUN_006183e0(*(int **)(*(int *)(*(int *)this + 4) + 4 + (int)this));
      iVar9 = iVar9 + 1;
      if (0xe < iVar9) goto LAB_00618624;
    }
    *(uint *)(*(int *)(*(int *)this + 4) + 8 + (int)this) =
         *(uint *)(*(int *)(*(int *)this + 4) + 8 + (int)this) | 1;
LAB_00618624:
    if (local_8 == 0) {
      *(uint *)(*(int *)(*(int *)this + 4) + 8 + (int)this) =
           *(uint *)(*(int *)(*(int *)this + 4) + 8 + (int)this) | 2;
      while (iVar9 != 0) {
        iVar9 = iVar9 + -1;
        cVar2 = *(char *)(iVar9 + param_1);
        piVar3 = *(int **)(*(int *)(*(int *)this + 4) + 4 + (int)this);
        if ((uint)piVar3[9] < (uint)piVar3[10]) {
          pcVar7 = (char *)(piVar3[10] - 1);
          piVar3[10] = (int)pcVar7;
          *pcVar7 = cVar2;
          iVar4 = (int)cVar2;
        }
        else {
          iVar4 = (**(code **)(*piVar3 + 0x24))((int)cVar2);
        }
        puVar1 = (uint *)(*(int *)(*(int *)this + 4) + 8 + (int)this);
        if (iVar4 == -1) {
          *puVar1 = *puVar1 | 4;
          break;
        }
        *puVar1 = *puVar1 & 0xfffffffe;
      }
      iVar9 = 0;
    }
    *(undefined1 *)(iVar9 + param_1) = 0;
    iVar4 = *(int *)(*(int *)(*(int *)this + 4) + 4 + (int)this);
    if (*(int *)(iVar4 + 0x30) < 0) {
      FUN_00615ef0((LPCRITICAL_SECTION)(iVar4 + 0x34));
    }
    param_1 = iVar9;
    if (*(int *)((int)this + *(int *)(*(int *)this + 4) + 0x34) < 0) {
      FUN_00615ef0((LPCRITICAL_SECTION)((int)this + *(int *)(*(int *)this + 4) + 0x38));
    }
  }
  if (param_1 == 0x10) {
    *(uint *)(*(int *)(*(int *)this + 4) + 8 + (int)this) =
         *(uint *)(*(int *)(*(int *)this + 4) + 8 + (int)this) | 2;
  }
  return uVar8;
}

