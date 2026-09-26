
uint __thiscall FUN_0051c5c0(void *this,uint param_1,int param_2)

{
  int iVar1;
  void *pvVar2;
  uint uVar3;
  uint uVar4;
  undefined4 *puVar5;
  undefined4 *puVar6;
  bool bVar7;
  undefined4 *local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00643086;
  local_c = ExceptionList;
  uVar4 = 1;
  if (*(int *)((int)this + 0x10) < (int)param_1) {
    ExceptionList = &local_c;
    local_10 = (undefined4 *)FUN_00618b70(param_1);
    bVar7 = local_10 == (undefined4 *)0x0;
    iVar1 = 0;
    if (!bVar7) {
      if (param_2 == 0) {
        *(undefined4 *)((int)this + 0x14) = 0;
      }
      else {
        uVar4 = *(uint *)((int)this + 0x14);
        if (uVar4 != 0) {
          puVar5 = *(undefined4 **)((int)this + 0x18);
          puVar6 = local_10;
          for (uVar3 = uVar4 >> 2; uVar3 != 0; uVar3 = uVar3 - 1) {
            *puVar6 = *puVar5;
            puVar5 = puVar5 + 1;
            puVar6 = puVar6 + 1;
          }
          for (uVar4 = uVar4 & 3; uVar4 != 0; uVar4 = uVar4 - 1) {
            *(undefined1 *)puVar6 = *(undefined1 *)puVar5;
            puVar5 = (undefined4 *)((int)puVar5 + 1);
            puVar6 = (undefined4 *)((int)puVar6 + 1);
          }
        }
      }
      iVar1 = FUN_0051c560((int)this);
      if (iVar1 != 0) {
        *(uint *)((int)this + 0x10) = param_1;
        *(undefined4 **)((int)this + 0x18) = local_10;
        local_10 = (undefined4 *)0x0;
      }
    }
    uVar4 = 0;
    if (iVar1 != 0) {
      pvVar2 = (void *)FUN_00618b70(0x5c);
      local_4 = 0;
      if (pvVar2 == (void *)0x0) {
        pvVar2 = (void *)0x0;
      }
      else {
        pvVar2 = FUN_00615ad0(pvVar2,*(char **)((int)this + 0x18),*(int *)((int)this + 0x10),1);
      }
      *(void **)((int)this + 8) = pvVar2;
      bVar7 = pvVar2 == (void *)0x0;
      local_4 = 0xffffffff;
      uVar4 = 0;
      if (!bVar7) {
        pvVar2 = (void *)FUN_00618b70(0x58);
        local_4 = 1;
        if (pvVar2 == (void *)0x0) {
          pvVar2 = (void *)0x0;
        }
        else {
          pvVar2 = FUN_00615c90(pvVar2,*(char **)((int)this + 0x18),*(int *)((int)this + 0x10),2,1);
        }
        local_4 = 0xffffffff;
        bVar7 = pvVar2 == (void *)0x0;
        uVar4 = (uint)!bVar7;
        *(void **)((int)this + 0xc) = pvVar2;
      }
    }
    if (local_10 != (undefined4 *)0x0) {
      FUN_00618b60((undefined *)local_10);
    }
    if (bVar7) {
      FUN_00619730();
    }
    *(uint *)((int)this + 4) = uVar4;
  }
  ExceptionList = local_c;
  return uVar4;
}

