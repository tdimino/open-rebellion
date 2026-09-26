
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __fastcall FUN_004bf490(int param_1)

{
  int iVar1;
  int iVar2;
  int iVar3;
  int iVar4;
  bool bVar5;
  float fVar6;
  int iVar7;
  int iVar8;
  uint *puVar9;
  
  iVar7 = *(int *)(param_1 + 0x2c);
  fVar6 = (float)*(int *)(iVar7 + 600);
  iVar8 = (*(int *)(iVar7 + 0x25c) - *(int *)(iVar7 + 0x1fc)) + *(int *)(iVar7 + 0x200);
  if (0 < *(int *)(iVar7 + 0x288)) {
    fVar6 = fVar6 / (float)*(int *)(iVar7 + 0x288);
  }
  iVar1 = *(int *)(iVar7 + 0x210);
  iVar2 = *(int *)(iVar7 + 0x1ec);
  iVar3 = *(int *)(iVar7 + 0x1e8);
  switch(*(undefined4 *)(param_1 + 0x5c)) {
  case 2:
    if (((iVar3 < *(int *)(iVar7 + 0x1ac) + 1) || (iVar2 < *(int *)(iVar7 + 0x1ac) / 2 + 1)) &&
       (((fVar6 < _DAT_0065c49c || (iVar8 <= iVar1)) && (*(int *)(iVar7 + 8) == 0)))) {
      iVar4 = *(int *)(iVar7 + 0x1c8);
      goto joined_r0x004bf627;
    }
    break;
  case 3:
    if (((fVar6 < _DAT_0065c49c) && (*(int *)(iVar7 + 8) == 0)) && (0 < iVar8)) {
LAB_004bf57d:
      iVar4 = *(int *)(iVar7 + 0x1ac);
      if ((iVar4 / 2 + 1 <= iVar3) && (((int)(iVar4 + (iVar4 >> 0x1f & 3U)) >> 2) + 1 <= iVar2))
      goto LAB_004bf630;
    }
    break;
  case 4:
    iVar4 = *(int *)(iVar7 + 8);
joined_r0x004bf627:
    if (iVar4 != 0) goto LAB_004bf630;
    break;
  case 5:
    if (((_DAT_0065c49c <= fVar6) && (*(int *)(iVar7 + 8) == 0)) && (iVar8 <= iVar1))
    goto LAB_004bf57d;
    break;
  case 6:
    if (((_DAT_0065c49c <= fVar6) && (*(int *)(iVar7 + 8) == 0)) && (0 < iVar8)) goto LAB_004bf630;
    break;
  case 7:
    if ((((iVar3 < *(int *)(iVar7 + 0x1ac) + 1) || (iVar2 < *(int *)(iVar7 + 0x1ac) / 2 + 1)) &&
        (0 < iVar8)) &&
       (((fVar6 < _DAT_0065c49c || (iVar8 <= iVar1)) && (*(int *)(iVar7 + 8) == 0)))) {
      iVar4 = *(int *)(iVar7 + 0x1c8);
      goto joined_r0x004bf627;
    }
  }
  *(undefined4 *)(param_1 + 0x5c) = 0;
LAB_004bf630:
  if (*(int *)(param_1 + 0x5c) == 0) {
    bVar5 = fVar6 < _DAT_0065c49c;
    *(uint *)(iVar7 + 4) = *(uint *)(iVar7 + 4) & 0xff3fffff;
    if ((bVar5) || (iVar7 = *(int *)(param_1 + 0x2c), *(int *)(iVar7 + 8) != 0)) {
      puVar9 = (uint *)(*(int *)(param_1 + 0x2c) + 4);
      *puVar9 = *puVar9 | 0x800000;
      iVar7 = *(int *)(param_1 + 0x2c);
      if (*(int *)(iVar7 + 8) == 0) {
        iVar4 = *(int *)(iVar7 + 0x1ac);
        if (((iVar3 < iVar4 / 2 + 1) || (iVar2 < ((int)(iVar4 + (iVar4 >> 0x1f & 3U)) >> 2) + 1)) &&
           (0 < *(int *)(iVar7 + 0x1c8))) {
          *(uint *)(param_1 + 0x5c) = ((iVar8 <= iVar1) - 1 & 5) + 2;
        }
        else {
          *(uint *)(param_1 + 0x5c) = ((iVar8 <= iVar1) - 1 & 2) + 1;
        }
      }
      else {
        *(undefined4 *)(param_1 + 0x5c) = 4;
      }
    }
    else if (iVar1 < iVar8) {
      *(undefined4 *)(param_1 + 0x5c) = 6;
    }
    else {
      iVar8 = *(int *)(iVar7 + 0x1ac);
      if (((iVar3 < iVar8 / 2 + 1) || (iVar2 < ((int)(iVar8 + (iVar8 >> 0x1f & 3U)) >> 2) + 1)) &&
         (0 < *(int *)(iVar7 + 0x1c8))) {
        *(undefined4 *)(param_1 + 0x5c) = 2;
      }
      else {
        *(uint *)(iVar7 + 4) = *(uint *)(iVar7 + 4) | 0x400000;
        *(undefined4 *)(param_1 + 0x5c) = 5;
      }
    }
  }
  if ((*(int **)(param_1 + 0x60) != (int *)0x0) &&
     (iVar7 = (**(code **)(**(int **)(param_1 + 0x60) + 0xc))(), iVar7 != *(int *)(param_1 + 0x5c)))
  {
    FUN_004e54f0(*(int *)(param_1 + 0x60));
    if (*(undefined4 **)(param_1 + 0x60) != (undefined4 *)0x0) {
      (**(code **)**(undefined4 **)(param_1 + 0x60))(1);
    }
    *(undefined4 *)(param_1 + 0x60) = 0;
  }
  if (*(int *)(param_1 + 0x60) == 0) {
    FUN_004bf7c0(param_1);
    if (*(int *)(param_1 + 0x60) != 0) {
      *(int *)(*(int *)(param_1 + 0x60) + 0x2c) = param_1 + 100;
      puVar9 = (uint *)(*(int *)(param_1 + 0x2c) + 4);
      *puVar9 = *puVar9 | 2;
    }
  }
  return;
}

