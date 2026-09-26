
int FUN_00548370(void *param_1,void *param_2)

{
  uint uVar1;
  uint uVar2;
  void *pvVar3;
  bool bVar4;
  int iVar5;
  uint *puVar6;
  void *pvVar7;
  undefined3 extraout_var;
  undefined3 extraout_var_00;
  void *this;
  int iVar8;
  int local_74;
  uint local_70;
  undefined4 local_6c [11];
  int local_40;
  undefined4 local_3c [12];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00647a80;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  iVar5 = FUN_00506f50();
  pvVar3 = param_1;
  local_70 = (uint)(iVar5 != 0);
  if (iVar5 != 0) {
    puVar6 = FUN_004025b0(param_1,(uint *)&param_1);
    uVar1 = *(uint *)(iVar5 + 0x68);
    uVar2 = *puVar6;
    FUN_00619730();
    if ((uVar1 == uVar2) && ((*(byte *)(iVar5 + 0xac) & 1) == 0)) {
      iVar5 = 1;
      goto LAB_005483ea;
    }
  }
  iVar5 = 0;
LAB_005483ea:
  pvVar7 = FUN_00525dc0(local_3c,pvVar3);
  local_4 = 0;
  bVar4 = FUN_00525a00((int)pvVar7);
  local_4 = 0xffffffff;
  FUN_00525e60(local_3c);
  local_74 = DAT_006b9050;
  bVar4 = FUN_00542990(iVar5,CONCAT31(extraout_var,bVar4),&local_74);
  if ((CONCAT31(extraout_var_00,bVar4) == 0) || (local_70 == 0)) {
    iVar5 = 0;
  }
  else {
    iVar5 = 1;
  }
  FUN_00525fe0(local_6c,pvVar3);
  local_4 = 1;
  FUN_00525930((int)local_6c);
  pvVar3 = param_2;
  iVar8 = local_74;
  while (local_74 = iVar8, local_40 != 0) {
    pvVar7 = pvVar3;
    this = (void *)FUN_00525f20((int)local_6c);
    iVar5 = FUN_004ee470(this,iVar8,pvVar7);
    FUN_005258f0((int)local_6c);
    iVar8 = local_74;
  }
  local_4 = 0xffffffff;
  FUN_00526080(local_6c);
  ExceptionList = local_c;
  return iVar5;
}

