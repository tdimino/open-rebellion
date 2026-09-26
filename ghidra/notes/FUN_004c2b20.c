
undefined4 __fastcall FUN_004c2b20(void *param_1)

{
  undefined4 *puVar1;
  bool bVar2;
  int iVar3;
  undefined2 *puVar4;
  uint uVar5;
  uint uVar6;
  uint uVar7;
  uint uVar8;
  
  bVar2 = false;
  uVar6 = 1;
  DAT_006b28cc = thunk_FUN_004fcee0();
  if (DAT_006b28c4 != 0) {
    FUN_004c3060(param_1);
    return 1;
  }
  DAT_006b28c4 = 0;
  while (DAT_006b28bc != 0) {
    if (bVar2) {
      DAT_006b28d0 = DAT_006b28cc;
      return 1;
    }
    bVar2 = false;
    if (*(uint *)((int)param_1 + 0x164) <= uVar6) break;
    uVar8 = *(uint *)(*(int *)((int)param_1 + 0x168) + uVar6 * 4);
    puVar1 = (undefined4 *)(*(int *)((int)param_1 + 0x168) + uVar6 * 4);
    if ((uVar8 == 0) || (uVar8 < DAT_006b28cc)) {
      *puVar1 = 0;
      uVar6 = uVar6 + 1;
    }
    else {
      *puVar1 = 0;
      if (*(uint *)(*(int *)((int)param_1 + 0x16c) + uVar6 * 4) < DAT_006b28cc) {
        *(uint *)(*(int *)((int)param_1 + 0x16c) + uVar6 * 4) = DAT_006b28cc + 0x3c;
        bVar2 = true;
        iVar3 = FUN_005f5500((void *)((int)param_1 + 0x138),uVar6);
        if (iVar3 != 0) {
          puVar4 = (undefined2 *)FUN_004c5420(iVar3);
          while (puVar4 != (undefined2 *)0x0) {
            if ((*(uint *)(puVar4 + 2) & 1) == 0) {
              uVar8 = *(uint *)(puVar4 + 6);
              uVar7 = *(uint *)(puVar4 + 4);
              uVar5 = CONCAT22((short)(uVar8 >> 0x10),*puVar4);
LAB_004c2bff:
              FUN_0041d660(uVar5,uVar7,uVar8,*(uint *)(puVar4 + 2) >> 1 & 1);
            }
            else if (((DAT_006b28b0 & 0x1000) != 0) && ((DAT_006b28b0 & 0x4000) == 0)) {
              uVar7 = *(uint *)(puVar4 + 4);
              uVar8 = *(uint *)(puVar4 + 6);
              uVar5 = CONCAT22((short)((uint)puVar4 >> 0x10),*puVar4);
              goto LAB_004c2bff;
            }
            puVar4 = (undefined2 *)FUN_004c5420(iVar3);
          }
        }
      }
    }
  }
  if (bVar2) {
    DAT_006b28d0 = DAT_006b28cc;
    return 1;
  }
  FUN_004c2dd0((int)param_1);
  return 1;
}

