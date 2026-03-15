
int __thiscall FUN_004fc080(void *this,int param_1,int param_2)

{
  int iVar1;
  bool bVar2;
  undefined3 extraout_var;
  int iVar3;
  undefined4 *puVar4;
  uint uVar5;
  char *pcVar6;
  char *pcVar7;
  uint uVar8;
  void *local_10;
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006401b0;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  local_10 = this;
  bVar2 = FUN_0053a010((int)this);
  iVar3 = CONCAT31(extraout_var,bVar2);
  FUN_004ece30(&local_10);
  uVar8 = 0;
  bVar2 = iVar3 == 0;
  local_4 = 0;
  if (!bVar2) {
    iVar3 = FUN_004f6b70(this,(uint *)&local_10);
    bVar2 = iVar3 == 0;
  }
  iVar1 = param_2;
  if (!bVar2) {
    FUN_004f8880(this,s_GameObjDestroyedNotif_006a9230,s_Destroyed_006a9248,param_1,(char *)0x0,
                 param_2);
    puVar4 = (undefined4 *)FUN_004ece30(&param_2);
    local_4._0_1_ = 1;
    iVar3 = FUN_0053fe40(0x302,this,&local_10,puVar4,iVar1);
    local_4 = (uint)local_4._1_3_ << 8;
    FUN_00619730();
  }
  if (iVar3 != 0) {
    if (((*(byte *)((int)this + 0x50) & 0x20) != 0) &&
       ((*(uint *)((int)this + 0x40) & 0xff0000) != 0)) {
      FUN_004f8880(this,s_GameObjDestroyedOnArrivalNotif_006a91fc,s_DestroyedOnArrival_006a921c,
                   param_1,(char *)0x0,iVar1);
      puVar4 = (undefined4 *)FUN_004ece30(&param_2);
      local_4._0_1_ = 2;
      iVar3 = FUN_0053fe40(0x303,this,&local_10,puVar4,iVar1);
      local_4 = (uint)local_4._1_3_ << 8;
      FUN_00619730();
    }
    if (iVar3 != 0) {
      pcVar6 = (char *)0x0;
      uVar5 = *(uint *)((int)this + 0x40) & 0xff;
      pcVar7 = (char *)0x0;
      if (uVar5 == 6) {
        uVar8 = 0x305;
        pcVar6 = s_GameObjDestroyedSabotageNotif_006a916c;
        pcVar7 = s_DestroyedSabotage_006a9158;
      }
      else if (uVar5 == 7) {
        uVar8 = 0x306;
        pcVar6 = s_GameObjDestroyedAssassinationNot_006a91a4;
        pcVar7 = s_DestroyedAssassination_006a918c;
      }
      else if (uVar5 == 0x15) {
        uVar8 = 0x304;
        pcVar6 = s_GameObjDestroyedAutoscrapNotif_006a91dc;
        pcVar7 = s_DestroyedAutoscrap_006a91c8;
      }
      if (uVar8 != 0) {
        FUN_004f8880(this,pcVar6,pcVar7,param_1,(char *)0x0,iVar1);
        puVar4 = (undefined4 *)FUN_004ece30(&param_1);
        local_4._0_1_ = 3;
        iVar3 = FUN_0053fe40(uVar8,this,&local_10,puVar4,iVar1);
        local_4 = (uint)local_4._1_3_ << 8;
        FUN_00619730();
      }
    }
  }
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return iVar3;
}

