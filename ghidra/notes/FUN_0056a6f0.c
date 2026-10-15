
uint __thiscall FUN_0056a6f0(void *param_1,void *param_2)

{
  int iVar1;
  bool bVar2;
  int iVar3;
  undefined3 extraout_var;
  int *piVar4;
  int iVar5;
  uint uVar6;
  uint uStack_18;
  int iStack_14;
  int iStack_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_0064b698;
  pvStack_c = ExceptionList;
  uVar6 = 1;
  if ((*(byte *)((int)param_1 + 0x88) & 1) != 0) {
    iStack_14 = *(int *)((int)param_1 + 0x5c);
    ExceptionList = &pvStack_c;
    FUN_0042dbe0(&uStack_18);
    iVar1 = *(int *)((int)param_1 + 100);
    iVar5 = 0;
    uStack_4 = 0;
    do {
      if (iStack_14 <= iVar5) break;
      iVar3 = FUN_0052d370(param_1,*(uint *)((int)param_1 + 0x24) >> 6 & 3,1);
      bVar2 = FUN_00559a60(iVar1,iVar3,&iStack_10,(int *)&uStack_18);
      uVar6 = CONCAT31(extraout_var,bVar2);
      if ((uVar6 != 0) && (iStack_10 != 0)) {
        piVar4 = FUN_004f7d50(param_1,&uStack_18,*(uint *)((int)param_1 + 0x24) >> 6 & 3,param_2,1,1
                             );
        uVar6 = (uint)(piVar4 != (int *)0x0);
      }
      iVar5 = iVar5 + 1;
    } while (uVar6 != 0);
    uStack_4 = 0xffffffff;
    FUN_00619730();
  }
  ExceptionList = pvStack_c;
  return uVar6;
}

