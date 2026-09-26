
uint __fastcall FUN_00516d50(int param_1)

{
  bool bVar1;
  undefined3 extraout_var;
  int iVar2;
  void *pvVar3;
  uint *puVar4;
  uint uVar5;
  undefined4 uVar6;
  void *pvVar7;
  uint *puVar8;
  uint uVar9;
  uint local_20;
  int local_1c;
  void *local_18;
  uint local_14 [2];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00642850;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  local_1c = param_1;
  local_18 = (void *)(param_1 + 0x90);
  bVar1 = FUN_005197f0(param_1);
  uVar9 = 0;
  if (CONCAT31(extraout_var,bVar1) != 0) {
    iVar2 = FUN_00516cd0(param_1);
    uVar9 = (uint)(iVar2 != 0);
    pvVar3 = (void *)FUN_00505ef0();
    pvVar7 = (void *)(param_1 + 0x90);
    if (pvVar3 != (void *)0x0) {
      puVar8 = (uint *)(param_1 + 0x8c);
      do {
        puVar4 = FUN_004ece40(puVar8);
        pvVar7 = local_18;
        param_1 = local_1c;
        if (puVar4 != (uint *)0x0) break;
        uVar5 = FUN_00509310((int)pvVar3);
        if (uVar5 != 0) {
          iVar2 = FUN_0050d000(pvVar3,0);
          if ((iVar2 == 0) || (uVar9 == 0)) {
            uVar9 = 0;
          }
          else {
            uVar9 = 1;
          }
          if ((*(uint *)((int)pvVar3 + 0x88) & 0x1000) != 0) {
            puVar4 = FUN_004025b0(pvVar3,&local_20);
            local_4 = 1;
            FUN_004f26d0(puVar8,puVar4);
            local_4 = 0xffffffff;
            FUN_00619730();
          }
        }
        local_14[0] = 0x90;
        local_14[1] = 0x98;
        local_4 = 0;
        pvVar3 = (void *)FUN_004f6010(pvVar3,local_14,1);
        local_4 = 0xffffffff;
        FUN_00619730();
        pvVar7 = local_18;
        param_1 = local_1c;
      } while (pvVar3 != (void *)0x0);
    }
    puVar8 = FUN_004ece40((uint *)(param_1 + 0x8c));
    if (puVar8 == (uint *)0x0) {
      *(undefined4 *)(param_1 + 0x70) = 0;
    }
    else {
      uVar6 = FUN_004fd2f0();
      *(undefined4 *)((int)pvVar7 + 0x14) = uVar6;
      *(undefined4 *)((int)pvVar7 + 0x18) = 0x226;
      pvVar3 = (void *)FUN_00505f60((uint *)(param_1 + 0x8c));
      if ((pvVar3 == (void *)0x0) || (uVar9 == 0)) {
        uVar9 = 0;
      }
      else {
        uVar9 = 1;
      }
      if (pvVar3 != (void *)0x0) {
        iVar2 = FUN_0050a580(pvVar3,1,pvVar7);
        if ((iVar2 == 0) || (uVar9 == 0)) {
          uVar9 = 0;
        }
        else {
          uVar9 = 1;
        }
      }
    }
  }
  ExceptionList = local_c;
  return uVar9;
}

