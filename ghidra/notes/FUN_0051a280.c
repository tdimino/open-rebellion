
int __thiscall FUN_0051a280(void *this,uint *param_1,void *param_2)

{
  void *pvVar1;
  int iVar2;
  bool bVar3;
  int iVar4;
  undefined3 extraout_var;
  uint *puVar5;
  void *pvVar6;
  uint *puVar7;
  uint *puVar8;
  int local_20;
  int local_1c;
  uint local_14;
  int local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00642da8;
  local_c = ExceptionList;
  local_20 = 1;
  ExceptionList = &local_c;
  *param_1 = 0;
  local_10 = 0;
  puVar5 = param_1;
  puVar7 = param_1;
  puVar8 = param_1;
  do {
    iVar2 = local_20;
    if (*param_1 != 0) {
      ExceptionList = local_c;
      return local_20;
    }
    local_1c = 0;
    iVar4 = thunk_FUN_005f5060((int)this + 0x100);
    if (iVar4 == 0) {
      iVar4 = thunk_FUN_005f5060((int)this + 0x110);
      if (iVar4 == 0) {
        iVar4 = thunk_FUN_005f5060((int)this + 0xe0);
        if (iVar4 == 0) {
          iVar4 = thunk_FUN_005f5060((int)this + 0xf0);
          if (iVar4 == 0) {
            iVar4 = thunk_FUN_005f5060((int)this + 0x120);
            if (iVar4 == 0) {
              iVar4 = thunk_FUN_005f5060((int)this + 0x130);
              if (iVar4 == 0) {
                pvVar1 = (void *)((int)this + 0x140);
                iVar4 = thunk_FUN_005f5060((int)pvVar1);
                pvVar6 = (void *)0x0;
                if (iVar4 != 0) {
                  puVar5 = (uint *)0x0;
                  local_4 = 0;
                  iVar4 = thunk_FUN_005f5060((int)pvVar1);
                  iVar4 = *(int *)(iVar4 + 0x24);
                  local_4 = 0xffffffff;
                  FUN_00619730();
                  pvVar6 = pvVar1;
                  if (iVar4 != -0x6dfffedf) {
                    local_4 = 1;
                    iVar4 = thunk_FUN_005f5060((int)pvVar1);
                    iVar4 = *(int *)(iVar4 + 0x24);
                    local_4 = 0xffffffff;
                    FUN_00619730();
                    if (iVar4 == -0x6ffffef7) {
                      puVar7 = (uint *)0x1;
                      puVar8 = (uint *)&DAT_00000002;
                      goto LAB_0051a47a;
                    }
                  }
                  puVar7 = (uint *)0x0;
                  puVar8 = (uint *)0x1;
                }
              }
              else {
                puVar7 = (uint *)0x0;
                puVar8 = (uint *)0x3;
                puVar5 = (uint *)0x0;
                pvVar6 = (void *)((int)this + 0x130);
              }
            }
            else {
              puVar7 = (uint *)0x1;
              puVar8 = (uint *)0x3;
              puVar5 = (uint *)0x0;
              pvVar6 = (void *)((int)this + 0x120);
            }
          }
          else {
            puVar7 = (uint *)0x1;
            puVar8 = (uint *)&DAT_00000002;
            puVar5 = (uint *)0x1;
            pvVar6 = (void *)((int)this + 0xf0);
          }
        }
        else {
          puVar7 = (uint *)0x1;
          puVar8 = (uint *)&DAT_00000002;
          puVar5 = (uint *)0x0;
          pvVar6 = (void *)((int)this + 0xe0);
        }
      }
      else {
        puVar7 = (uint *)0x1;
        puVar8 = (uint *)0x1;
        puVar5 = (uint *)0x1;
        pvVar6 = (void *)((int)this + 0x110);
      }
    }
    else {
      puVar7 = (uint *)0x1;
      puVar8 = (uint *)0x1;
      puVar5 = (uint *)0x0;
      pvVar6 = (void *)((int)this + 0x100);
    }
LAB_0051a47a:
    if (pvVar6 != (void *)0x0) {
      local_1c = thunk_FUN_005f5060((int)pvVar6);
      FUN_004f26d0(&local_14,(undefined4 *)(local_1c + 0x24));
      local_4 = 2;
      bVar3 = FUN_0051bc70(&local_14,puVar7,puVar8,(int)puVar5,param_2);
      if ((CONCAT31(extraout_var,bVar3) == 0) || (local_20 = 1, iVar2 == 0)) {
        local_20 = 0;
      }
      FUN_005f54a0(pvVar6,*(uint *)(local_1c + 0x18));
      local_4 = 0xffffffff;
      FUN_00619730();
    }
    local_10 = local_10 + 1;
    *param_1 = (uint)(local_1c == 0);
    if (0x27 < local_10) {
      ExceptionList = local_c;
      return local_20;
    }
  } while( true );
}

