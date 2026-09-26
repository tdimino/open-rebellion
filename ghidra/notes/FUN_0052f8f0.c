
int __thiscall FUN_0052f8f0(void *this,void *param_1)

{
  int iVar1;
  int iVar2;
  bool bVar3;
  bool bVar4;
  int *piVar5;
  void *this_00;
  void *pvVar6;
  int iVar7;
  int iVar8;
  int local_44;
  int local_40;
  int local_3c [2];
  uint local_34;
  undefined4 local_30;
  undefined1 local_2c [8];
  undefined1 local_24 [8];
  undefined1 local_1c [8];
  undefined1 local_14 [8];
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00645380;
  local_c = ExceptionList;
  piVar5 = (int *)((int)this + 0x68);
  iVar8 = *(int *)((int)this + 0x74) - *(int *)((int)this + 0x6c);
  bVar4 = true;
  iVar7 = 1;
  if (*piVar5 - *(int *)((int)this + 0x6c) < iVar8) {
    iVar8 = *piVar5 - *(int *)((int)this + 0x6c);
  }
  if (iVar8 != 0) {
    bVar3 = iVar8 < 1;
    if (bVar3) {
      iVar8 = -iVar8;
    }
    ExceptionList = &local_c;
    FUN_00520580(local_3c,piVar5);
    local_4 = 0;
    if (iVar8 != 0) {
      while (bVar4) {
        bVar4 = false;
        this_00 = (void *)FUN_00506960();
        while ((iVar7 != 0 && (this_00 != (void *)0x0))) {
          if (iVar8 == 0) goto LAB_0052faf3;
          if ((((*(byte *)((int)this_00 + 0x50) & 1) != 0) &&
              ((*(byte *)((int)this_00 + 0x60) & 1) == 0)) &&
             (((*(uint *)((int)this + 0x24) ^ *(uint *)((int)this_00 + 0x24)) & 0xc0) == 0)) {
            if (bVar3) {
              iVar7 = FUN_0055a960(this_00,iVar8,local_3c,&local_40,param_1);
              if (iVar7 != 0) {
                iVar8 = iVar8 - local_40;
                piVar5 = (int *)FUN_0051c900(local_1c,0,local_40);
                local_4._0_1_ = 3;
                FUN_0051c9e0(local_3c,piVar5);
                local_4 = (uint)local_4._1_3_ << 8;
                FUN_00619730();
                pvVar6 = FUN_0052e740(this_00,local_14);
                iVar1 = *(int *)((int)pvVar6 + 4);
                FUN_00619730();
                if (iVar1 != 0) goto LAB_0052faa8;
              }
            }
            else {
              iVar7 = FUN_0055a820(this_00,iVar8,local_3c,&local_44,param_1);
              if (iVar7 != 0) {
                iVar8 = iVar8 - local_44;
                piVar5 = (int *)FUN_0051c900(local_2c,0,local_44);
                local_4._0_1_ = 2;
                FUN_0051c9c0(local_3c,piVar5);
                local_4 = (uint)local_4._1_3_ << 8;
                FUN_00619730();
                piVar5 = FUN_0052e740(this_00,local_24);
                iVar1 = *piVar5;
                iVar2 = piVar5[1];
                FUN_00619730();
                if (iVar1 != iVar2) {
LAB_0052faa8:
                  bVar4 = true;
                }
              }
            }
          }
          local_34 = 0x2d;
          local_30 = 0x2e;
          local_4._0_1_ = 1;
          this_00 = (void *)FUN_004f6010(this_00,&local_34,1);
          local_4 = (uint)local_4._1_3_ << 8;
          FUN_00619730();
        }
        if (iVar8 == 0) break;
      }
    }
LAB_0052faf3:
    FUN_0052ee10(this,local_3c,param_1);
    local_4 = 0xffffffff;
    FUN_00619730();
  }
  ExceptionList = local_c;
  return iVar7;
}

