
undefined4 __fastcall FUN_004c0a60(void *param_1)

{
  undefined4 *puVar1;
  uint uVar2;
  bool bVar3;
  int iVar4;
  undefined2 *puVar5;
  uint uVar6;
  
  bVar3 = false;
  uVar6 = 1;
  DAT_006b28cc = thunk_FUN_004fcee0();
  if (DAT_006b28c4 != 0) {
    FUN_004c0f60(param_1);
    return 1;
  }
  DAT_006b28c4 = 0;
  while (DAT_006b28bc != 0) {
    if (bVar3) {
      DAT_006b28d0 = DAT_006b28cc;
      return 1;
    }
    bVar3 = false;
    if (*(uint *)((int)param_1 + 0x164) <= uVar6) break;
    uVar2 = *(uint *)(*(int *)((int)param_1 + 0x168) + uVar6 * 4);
    puVar1 = (undefined4 *)(*(int *)((int)param_1 + 0x168) + uVar6 * 4);
    if ((uVar2 == 0) || (uVar2 < DAT_006b28cc)) {
      *puVar1 = 0;
      uVar6 = uVar6 + 1;
    }
    else {
      *puVar1 = 0;
      if (*(uint *)(*(int *)((int)param_1 + 0x16c) + uVar6 * 4) < DAT_006b28cc) {
        *(uint *)(*(int *)((int)param_1 + 0x16c) + uVar6 * 4) = DAT_006b28cc + 0x3c;
        bVar3 = true;
        iVar4 = FUN_005f5500((void *)((int)param_1 + 0x138),uVar6);
        if (iVar4 != 0) {
          puVar5 = (undefined2 *)FUN_004c5420(iVar4);
          while (puVar5 != (undefined2 *)0x0) {
            if (((*(uint *)(puVar5 + 2) & 1) == 0) ||
               (((DAT_006b28b0 & 0x1000) != 0 && ((DAT_006b28b0 & 0x4000) == 0)))) {
              FUN_0041d660(CONCAT22((short)(*(uint *)(puVar5 + 6) >> 0x10),*puVar5),
                           *(uint *)(puVar5 + 4),*(uint *)(puVar5 + 6),
                           *(uint *)(puVar5 + 2) >> 1 & 1);
            }
            puVar5 = (undefined2 *)FUN_004c5420(iVar4);
          }
        }
      }
    }
  }
  if (bVar3) {
    DAT_006b28d0 = DAT_006b28cc;
    return 1;
  }
  FUN_004c0d00((int)param_1);
  return 1;
}

