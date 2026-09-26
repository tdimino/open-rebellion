
int * __cdecl FUN_00584e60(int *param_1,int *param_2,int *param_3)

{
  int *this;
  int iVar1;
  bool bVar2;
  bool bVar3;
  bool bVar4;
  bool bVar5;
  char cVar6;
  uint uVar7;
  char cVar8;
  int iVar9;
  bool bVar10;
  bool bVar11;
  uint local_4;
  
  iVar9 = *param_3;
  bVar10 = false;
  iVar1 = *(int *)(*param_2 + 4);
  if (*(int *)((int)param_2 + iVar1 + 0x34) < 0) {
    FUN_00615ee0((LPCRITICAL_SECTION)((int)param_2 + iVar1 + 0x38));
  }
  *(undefined4 *)((int)param_2 + iVar1 + 8) = 0;
  if (*(int *)((int)param_2 + iVar1 + 0x34) < 0) {
    FUN_00615ef0((LPCRITICAL_SECTION)((int)param_2 + iVar1 + 0x38));
  }
  this = param_2 + 3;
  FUN_00616900(this,0);
  FUN_00616c30(param_2,0);
  bVar4 = true;
  bVar5 = false;
  bVar11 = false;
  cVar8 = '\0';
  if (iVar9 == 0) {
LAB_00584ed4:
    uVar7 = FUN_00617330(param_1);
    cVar6 = (char)uVar7;
    local_4 = CONCAT31(local_4._1_3_,cVar6);
    bVar2 = true;
    if ((*(int *)(*(int *)(*param_1 + 4) + 8 + (int)param_1) == 0) && (cVar6 != '\n')) {
      iVar9 = 0;
    }
    else {
      iVar9 = 1;
    }
    *param_3 = iVar9;
    if (iVar9 == 0) {
      if (bVar4) {
        bVar10 = cVar6 == '\"';
        bVar2 = !bVar10;
        bVar4 = false;
        bVar11 = cVar6 == ',';
      }
      else if (bVar10) {
        if (cVar6 == '\"') {
          if (cVar8 == '\"') {
            bVar5 = false;
          }
          else {
            bVar2 = false;
            bVar5 = true;
          }
        }
        else if ((cVar8 == '\"') && (bVar5)) {
          bVar11 = true;
        }
      }
      else {
        bVar11 = cVar6 == ',';
      }
      if (bVar11) goto LAB_00584f91;
      bVar3 = false;
    }
    else {
LAB_00584f91:
      bVar3 = true;
    }
    if (!bVar3) {
      cVar8 = cVar6;
      if (bVar2) {
        FUN_00615f00(this,local_4);
      }
      goto LAB_00584ed4;
    }
  }
  FUN_00615f00(this,0);
  return param_2;
}

