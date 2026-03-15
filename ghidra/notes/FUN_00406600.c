
void FUN_00406600(void)

{
  char *pcVar1;
  bool bVar2;
  int *piVar3;
  DWORD DVar4;
  char *pcVar5;
  char *pcVar6;
  int iVar7;
  undefined3 extraout_var;
  undefined3 extraout_var_00;
  char *pcVar8;
  uint uVar9;
  char *pcVar10;
  char *local_10;
  char *local_c;
  char *local_8;
  char *local_4;
  
  piVar3 = (int *)FUN_00619540(s_data0_reb_006a7e78,&DAT_006a7e84);
  if (piVar3 == (int *)0x0) {
    FUN_00618ed0((undefined1 *)&lpCaption_006b0bf8,s_Error__CD_Not_Found_006a7e64);
    FUN_00618ed0((undefined1 *)&lpText_006b1000,s_The_CD_ROM_could_not_be_found__p_006a7dec);
    FUN_00618ed0((undefined1 *)&lpCaption_006b0e00,s_Error__Versions_Incompatible_006a7dcc);
    FUN_00618ed0((undefined1 *)&lpText_006b07f8,s_Your_version_of_the_game_is_olde_006a7d38);
    FUN_00618ed0((undefined1 *)&lpText_006b09f8,s_Your_opponent_s_version_of_the_g_006a7ca4);
    DAT_006b0df8 = 1;
    bVar2 = FUN_00406850();
    DAT_006b0df8 = CONCAT31(extraout_var_00,bVar2);
    return;
  }
  FUN_00619420(piVar3,0,2);
  DVar4 = FUN_00619240((char *)piVar3);
  FUN_00619420(piVar3,0,0);
  pcVar5 = (char *)FUN_00619170(DVar4);
  FUN_00618ff0(pcVar5,DVar4,1,piVar3);
  FUN_00618f40(piVar3);
  pcVar6 = (char *)(uint)(*pcVar5 != '#');
  iVar7 = 0;
  uVar9 = 0;
  pcVar10 = pcVar6;
  pcVar1 = pcVar5;
  if (0 < (int)DVar4) {
    do {
      pcVar8 = pcVar1 + 1;
      if (pcVar6 != (char *)0x0) {
        pcVar5[iVar7] = pcVar5[iVar7] ^ 0xdd;
      }
      if (pcVar5[iVar7] == '\"') {
        uVar9 = uVar9 + 1;
        if (uVar9 == 1) {
          pcVar10 = pcVar8;
        }
        if (uVar9 == 3) {
          local_10 = pcVar8;
        }
        if (uVar9 == 5) {
          local_4 = pcVar8;
        }
        if (uVar9 == 7) {
          local_c = pcVar8;
        }
        if (uVar9 == 9) {
          local_8 = pcVar8;
        }
        if ((uVar9 & 1) == 0) {
          pcVar5[iVar7] = '\0';
        }
      }
      if (pcVar5[iVar7] == '\\') {
        pcVar5[iVar7] = ' ';
        *pcVar8 = '\n';
        iVar7 = iVar7 + 1;
        pcVar8 = pcVar1 + 2;
      }
      iVar7 = iVar7 + 1;
      pcVar1 = pcVar8;
    } while (iVar7 < (int)DVar4);
  }
  FUN_00618ed0((undefined1 *)&lpText_006b1000,local_10);
  FUN_00618ed0((undefined1 *)&lpCaption_006b0bf8,pcVar10);
  FUN_00618ed0((undefined1 *)&lpText_006b07f8,local_c);
  FUN_00618ed0((undefined1 *)&lpText_006b09f8,local_8);
  FUN_00618ed0((undefined1 *)&lpCaption_006b0e00,local_4);
  FUN_00618e60(pcVar5);
  bVar2 = FUN_00406850();
  DAT_006b0df8 = CONCAT31(extraout_var,bVar2);
  return;
}

