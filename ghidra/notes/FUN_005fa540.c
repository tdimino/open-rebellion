
undefined4 __fastcall FUN_005fa540(int param_1)

{
  int *piVar1;
  undefined4 *puVar2;
  int iVar3;
  uint uVar4;
  uint unaff_ESI;
  undefined4 *puVar5;
  undefined4 local_4;
  
  piVar1 = *(int **)(param_1 + 0x108);
  if ((piVar1 != (int *)0x0) && (*(int *)(param_1 + 0x24) != 0)) {
    local_4 = 0;
    (**(code **)(*piVar1 + 0x58))(piVar1,0,&local_4);
    puVar2 = (undefined4 *)FUN_00618b70(unaff_ESI);
    puVar5 = puVar2;
    for (uVar4 = unaff_ESI >> 2; uVar4 != 0; uVar4 = uVar4 - 1) {
      *puVar5 = 0;
      puVar5 = puVar5 + 1;
    }
    for (uVar4 = unaff_ESI & 3; uVar4 != 0; uVar4 = uVar4 - 1) {
      *(undefined1 *)puVar5 = 0;
      puVar5 = (undefined4 *)((int)puVar5 + 1);
    }
    iVar3 = (**(code **)(**(int **)(param_1 + 0x108) + 0x58))
                      (*(int **)(param_1 + 0x108),puVar2,&stack0xfffffff0);
    if (iVar3 == 0) {
      puVar2[10] = DAT_006ac294;
      puVar2[1] = puVar2[1] | 0x21;
      (**(code **)(**(int **)(param_1 + 0x108) + 0x7c))(*(int **)(param_1 + 0x108),puVar2,0);
    }
    FUN_00618b60((undefined *)puVar2);
  }
  return 0;
}

