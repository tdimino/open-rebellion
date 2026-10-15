
undefined4 __thiscall FUN_00404360(void *param_1,int *param_2,undefined4 *param_3)

{
  undefined4 *puVar1;
  int iVar2;
  int iVar3;
  undefined4 uStack_4;
  
  puVar1 = param_3;
  *param_3 = 0;
  uStack_4 = 0;
  iVar3 = 1;
  param_3 = (undefined4 *)0x0;
  if (0 < *(int *)((int)param_1 + 0x48)) {
    do {
      if (*(int *)(*(int *)((int)param_1 + 0x5c) + (int)param_3 * 4) != 0) {
        *puVar1 = 1;
        FUN_0041c7a0(*(void **)(*(int *)((int)param_1 + 0x5c) + (int)param_3 * 4),
                     *(uint **)(*(int *)((int)param_1 + 0x60) + (int)param_3 * 4));
        FUN_0041ca10(*(void **)(*(int *)((int)param_1 + 0x5c) + (int)param_3 * 4),param_2,iVar3);
        iVar3 = 0;
        *(undefined4 *)(*(int *)((int)param_1 + 0x5c) + (int)param_3 * 4) =
             *(undefined4 *)(*(int *)(*(int *)((int)param_1 + 0x5c) + (int)param_3 * 4) + 0x10);
      }
      if (*(int *)((int)param_1 + 100) != 0) {
        FUN_00404020(param_1,1);
        FUN_00404040(param_1,1);
        uStack_4 = 1;
      }
      if ((*(int *)(*(int *)((int)param_1 + 0x5c) + (int)param_3 * 4) == 0) &&
         (iVar2 = FUN_00403fe0((int)param_1), iVar2 == 0)) {
        uStack_4 = 1;
      }
      param_3 = (undefined4 *)((int)param_3 + 1);
    } while ((int)param_3 < *(int *)((int)param_1 + 0x48));
  }
  return uStack_4;
}

