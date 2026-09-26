
uint __thiscall FUN_00529dd0(void *this,void *param_1)

{
  uint uVar1;
  int iVar2;
  int iVar3;
  int *piVar4;
  int iVar5;
  int iVar6;
  void *pvVar7;
  int iVar8;
  uint local_40;
  int local_38;
  int local_34;
  undefined4 local_2c [8];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006448c8;
  local_c = ExceptionList;
  iVar8 = 0;
  local_38 = 0;
  iVar5 = 0;
  iVar6 = 0;
  local_34 = 0;
  ExceptionList = &local_c;
  iVar2 = FUN_0052bed0((int)this);
  local_40 = (uint)(iVar2 != 0);
  if (local_40 != 0) {
    FUN_0052cda0(local_2c,iVar2);
    local_4 = 0;
    FUN_00513120((int)local_2c);
    iVar3 = FUN_0052bed0((int)local_2c);
    while (iVar3 != 0) {
      piVar4 = (int *)FUN_0052bed0((int)local_2c);
      iVar3 = FUN_0052bde0(this,piVar4);
      if ((iVar3 != 0) && (iVar8 = iVar8 + 1, (*(byte *)(piVar4 + 0x18) & 1) == 0)) {
        switch(piVar4[0x16]) {
        case 1:
          iVar5 = iVar5 + 1;
          break;
        case 2:
          local_38 = local_38 + 1;
          break;
        case 3:
          iVar6 = iVar6 + 1;
          break;
        case 4:
          local_34 = local_34 + 1;
        }
      }
      FUN_005130d0((int)local_2c);
      iVar3 = FUN_0052bed0((int)local_2c);
    }
    local_4 = 0xffffffff;
    FUN_0052cec0(local_2c);
  }
  if (local_40 != 0) {
    local_38 = iVar6 + iVar5 + local_38;
    if (iVar8 == 0) {
      local_40 = FUN_0052a2c0(this,param_1);
    }
    else {
      iVar8 = (*(int *)((int)this + 0x6c) - *(int *)((int)this + 0x60)) - *(int *)((int)this + 0x5c)
      ;
      if (local_38 < iVar8) {
        iVar5 = iVar8 - local_38;
        if (local_34 < iVar8 - local_38) {
          iVar5 = local_34;
        }
        do {
          uVar1 = local_40;
          pvVar7 = (void *)0x0;
          FUN_0052cda0(local_2c,iVar2);
          local_4 = 1;
          FUN_00513120((int)local_2c);
          iVar8 = FUN_0052bed0((int)local_2c);
          while (iVar8 != 0) {
            piVar4 = (int *)FUN_0052bed0((int)local_2c);
            iVar8 = FUN_0052bde0(this,piVar4);
            if (((iVar8 != 0) &&
                (iVar8 = FUN_0052bed0((int)local_2c), (*(uint *)(iVar8 + 0x60) & 1) == 0)) &&
               (iVar8 = FUN_0052bed0((int)local_2c), *(int *)(iVar8 + 0x58) == 4)) {
              if (pvVar7 != (void *)0x0) {
                iVar8 = FUN_0052bed0((int)local_2c);
                iVar8 = FUN_00520b70(iVar8);
                iVar6 = FUN_00520b70((int)pvVar7);
                if (iVar6 <= iVar8) goto LAB_00529f95;
              }
              pvVar7 = (void *)FUN_0052bed0((int)local_2c);
            }
LAB_00529f95:
            FUN_005130d0((int)local_2c);
            iVar8 = FUN_0052bed0((int)local_2c);
          }
          if (pvVar7 != (void *)0x0) {
            iVar8 = FUN_0053aac0(pvVar7,param_1);
            if ((iVar8 == 0) || (local_40 = 1, uVar1 == 0)) {
              local_40 = 0;
            }
            iVar5 = iVar5 + -1;
          }
          local_4 = 0xffffffff;
          FUN_0052cec0(local_2c);
        } while ((pvVar7 != (void *)0x0) && (iVar5 != 0));
      }
      else if (iVar8 < local_38) {
        local_38 = local_38 - iVar8;
        iVar8 = 1;
        do {
          if (local_38 == 0) {
            ExceptionList = local_c;
            return local_40;
          }
          do {
            uVar1 = local_40;
            pvVar7 = (void *)0x0;
            FUN_0052cda0(local_2c,iVar2);
            local_4 = 2;
            FUN_00513120((int)local_2c);
            iVar5 = FUN_0052bed0((int)local_2c);
            while (iVar5 != 0) {
              piVar4 = (int *)FUN_0052bed0((int)local_2c);
              iVar5 = FUN_0052bde0(this,piVar4);
              if (((iVar5 != 0) &&
                  (iVar5 = FUN_0052bed0((int)local_2c), (*(uint *)(iVar5 + 0x60) & 1) == 0)) &&
                 (iVar5 = FUN_0052bed0((int)local_2c), *(int *)(iVar5 + 0x58) == iVar8)) {
                if (pvVar7 != (void *)0x0) {
                  iVar5 = FUN_0052bed0((int)local_2c);
                  iVar5 = FUN_00520b70(iVar5);
                  iVar6 = FUN_00520b70((int)pvVar7);
                  if (iVar5 <= iVar6) goto LAB_0052a0a5;
                }
                pvVar7 = (void *)FUN_0052bed0((int)local_2c);
              }
LAB_0052a0a5:
              FUN_005130d0((int)local_2c);
              iVar5 = FUN_0052bed0((int)local_2c);
            }
            if (pvVar7 != (void *)0x0) {
              iVar5 = FUN_0053ab10(pvVar7,param_1);
              if ((iVar5 == 0) || (local_40 = 1, uVar1 == 0)) {
                local_40 = 0;
              }
              local_38 = local_38 + -1;
            }
            local_4 = 0xffffffff;
            FUN_0052cec0(local_2c);
          } while ((pvVar7 != (void *)0x0) && (local_38 != 0));
          iVar8 = iVar8 + 1;
        } while (iVar8 < 4);
      }
    }
  }
  ExceptionList = local_c;
  return local_40;
}

