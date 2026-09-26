
uint __thiscall FUN_0058f560(void *this,int *param_1)

{
  bool bVar1;
  undefined3 extraout_var;
  int *piVar2;
  int *piVar3;
  undefined4 *puVar4;
  uint uVar5;
  int iVar6;
  uint *puVar7;
  undefined1 **ppuVar8;
  int local_90;
  uint local_8c;
  int local_88;
  int local_84;
  uint local_80;
  undefined1 *local_7c;
  int local_78;
  undefined1 *local_74;
  int local_70 [5];
  undefined4 local_5c [20];
  void *pvStack_c;
  undefined1 *puStack_8;
  uint local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00650403;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  FUN_00615db0(local_70,1);
  local_4 = 0;
  local_90 = 0;
  bVar1 = FUN_0058fa30(this,param_1,&local_90);
  FUN_0042dbe0(&local_8c);
  local_4 = CONCAT31(local_4._1_3_,1);
  uVar5 = 0;
  if (CONCAT31(extraout_var,bVar1) != 0) {
    puVar4 = (undefined4 *)((int)this + 0x24);
    piVar2 = FUN_00584e60(param_1,local_70,&local_90);
    FUN_00617610(piVar2,puVar4);
    piVar2 = &local_84;
    piVar3 = FUN_00584e60(param_1,local_70,&local_90);
    FUN_00617760(piVar3,piVar2);
    puVar7 = &local_80;
    piVar2 = FUN_00584e60(param_1,local_70,&local_90);
    FUN_00617760(piVar2,(int *)puVar7);
    local_8c = local_8c & 0xffffff | local_84 << 0x18;
    local_7c = &stack0xffffff5c;
    local_8c = local_8c ^ (local_8c ^ local_80) & 0xffffff;
    FUN_004f26d0(&stack0xffffff5c,&local_8c);
    puVar4 = FUN_004f42a0((void *)((int)this + 0x28));
    uVar5 = (uint)(puVar4 != (undefined4 *)0x0);
  }
  local_78 = DAT_0066a9b4;
  for (local_88 = 0; (uVar5 != 0 && (local_88 < local_78)); local_88 = local_88 + 1) {
    piVar2 = &local_84;
    piVar3 = FUN_00584e60(param_1,local_70,&local_90);
    FUN_00617760(piVar3,piVar2);
    puVar7 = &local_80;
    piVar2 = FUN_00584e60(param_1,local_70,&local_90);
    FUN_00617760(piVar2,(int *)puVar7);
    ppuVar8 = &local_7c;
    piVar2 = FUN_00584e60(param_1,local_70,&local_90);
    FUN_00617610(piVar2,ppuVar8);
    local_8c = local_8c & 0xffffff | local_84 << 0x18;
    local_8c = local_8c ^ (local_8c ^ local_80) & 0xffffff;
    for (iVar6 = 0; (uVar5 != 0 && (iVar6 < (int)local_7c)); iVar6 = iVar6 + 1) {
      local_74 = &stack0xffffff5c;
      FUN_004f26d0(&stack0xffffff5c,&local_8c);
      puVar4 = FUN_004f42a0((void *)((int)this + 0x28));
      uVar5 = (uint)(puVar4 != (undefined4 *)0x0);
    }
  }
  FUN_00584fd0(param_1,&local_90);
  local_4 = local_4 & 0xffffff00;
  FUN_00619730();
  local_4 = 0xffffffff;
  FUN_00615ea0((int)local_5c);
  FUN_00615440(local_5c);
  ExceptionList = pvStack_c;
  return uVar5;
}

