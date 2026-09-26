
int __thiscall FUN_0051cec0(void *this,int *param_1)

{
  int iVar1;
  undefined3 extraout_var;
  int iVar2;
  undefined3 extraout_var_00;
  undefined3 extraout_var_01;
  undefined3 extraout_var_02;
  undefined3 extraout_var_03;
  undefined3 extraout_var_04;
  undefined3 extraout_var_05;
  undefined3 extraout_var_06;
  undefined3 extraout_var_07;
  undefined3 extraout_var_08;
  undefined3 extraout_var_09;
  undefined3 extraout_var_10;
  undefined4 *puVar3;
  bool bVar4;
  int local_2c;
  undefined4 auStack_28 [7];
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_00643130;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  iVar1 = FUN_0051e5b0((int)this);
  bVar4 = iVar1 == 0;
  iVar1 = 0;
  if (!bVar4) {
    iVar1 = FUN_0051e320((int)this);
    bVar4 = iVar1 == 0;
  }
  if (!bVar4) {
    bVar4 = FUN_005f4d40(param_1);
    iVar1 = CONCAT31(extraout_var,bVar4);
  }
  if (iVar1 != 0) {
    FUN_00568a30(param_1);
    FUN_005f4d90(param_1,(int)this + 4);
    FUN_005f4d90(param_1,(int)this + 8);
    FUN_005f4d90(param_1,(int)this + 0xc);
    FUN_005f4d90(param_1,(int)this + 0x10);
    FUN_005f4d90(param_1,(int)this + 0x14);
    FUN_005f4d90(param_1,(int)this + 0x18);
    FUN_005f4d90(param_1,(int)this + 0x1c);
    FUN_005f4d90(param_1,(int)this + 0x20);
    FUN_005f4d90(param_1,(int)this + 0x24);
    FUN_005f4d90(param_1,(int)this + 0x28);
    FUN_005f4d90(param_1,(int)this + 0x2c);
    FUN_005f4d90(param_1,(int)this + 0x30);
    FUN_005f4d90(param_1,(int)this + 0x34);
    FUN_005f4d90(param_1,(int)this + 0x38);
    FUN_005f4d90(param_1,(int)this + 0x3c);
    FUN_005f4d90(param_1,(int)this + 0x40);
    FUN_005f4d90(param_1,(int)this + 0x44);
    FUN_005f4d90(param_1,(int)this + 0x48);
    FUN_005f4d90(param_1,(int)this + 0x4c);
    if (iVar1 != 0) {
      iVar2 = FUN_00539960((void *)((int)this + 0x50),param_1);
      iVar1 = 0;
      if (iVar2 != 0) {
        iVar2 = FUN_00539960((void *)((int)this + 0x6c),param_1);
        iVar1 = 0;
        if (iVar2 != 0) {
          iVar2 = FUN_00539960((void *)((int)this + 0x88),param_1);
          iVar1 = 0;
          if (iVar2 != 0) {
            bVar4 = FUN_005f4d40(param_1);
            iVar1 = 0;
            if (CONCAT31(extraout_var_00,bVar4) != 0) {
              FUN_00536f00(*(void **)((int)this + 0xa4),param_1);
              bVar4 = FUN_005f4d40(param_1);
              iVar1 = 0;
              if (CONCAT31(extraout_var_01,bVar4) != 0) {
                iVar2 = (**(code **)(**(int **)((int)this + 0xa8) + 0xc))(param_1);
                iVar1 = 0;
                if (iVar2 != 0) {
                  bVar4 = FUN_005f4d40(param_1);
                  iVar1 = 0;
                  if (CONCAT31(extraout_var_02,bVar4) != 0) {
                    iVar2 = (**(code **)(**(int **)((int)this + 0xac) + 0xc))(param_1);
                    iVar1 = 0;
                    if (iVar2 != 0) {
                      bVar4 = FUN_005f4d40(param_1);
                      iVar1 = 0;
                      if (CONCAT31(extraout_var_03,bVar4) != 0) {
                        iVar2 = (**(code **)(**(int **)((int)this + 0xb0) + 0xc))(param_1);
                        iVar1 = 0;
                        if (iVar2 != 0) {
                          bVar4 = FUN_005f4d40(param_1);
                          iVar1 = 0;
                          if (CONCAT31(extraout_var_04,bVar4) != 0) {
                            bVar4 = FUN_005688d0(*(void **)((int)this + 0xb4),param_1);
                            iVar1 = 0;
                            if (CONCAT31(extraout_var_05,bVar4) != 0) {
                              bVar4 = FUN_005f4d40(param_1);
                              iVar1 = 0;
                              if (CONCAT31(extraout_var_06,bVar4) != 0) {
                                bVar4 = FUN_005688d0(*(void **)((int)this + 0xb8),param_1);
                                iVar1 = 0;
                                if (CONCAT31(extraout_var_07,bVar4) != 0) {
                                  bVar4 = FUN_005f4d40(param_1);
                                  iVar1 = 0;
                                  if (CONCAT31(extraout_var_08,bVar4) != 0) {
                                    iVar2 = (**(code **)(**(int **)((int)this + 0xbc) + 0xc))
                                                      (param_1);
                                    iVar1 = 0;
                                    if (iVar2 != 0) {
                                      bVar4 = FUN_005f4d40(param_1);
                                      iVar1 = 0;
                                      if (CONCAT31(extraout_var_09,bVar4) != 0) {
                                        iVar1 = FUN_00513a50(param_1,(int *)((int)this + 0xc0));
                                      }
                                    }
                                  }
                                }
                              }
                            }
                          }
                        }
                      }
                    }
                  }
                }
              }
            }
          }
        }
      }
    }
  }
  local_2c = 1;
  if (iVar1 != 0) {
    FUN_005f4d90(param_1,&local_2c);
    if (iVar1 != 0) {
      if (local_2c == 0) {
        FUN_005686a0(auStack_28);
        uStack_4 = 0;
        FUN_00568770(auStack_28,param_1);
        FUN_00568770(auStack_28,param_1);
        FUN_00568770(auStack_28,param_1);
        FUN_00568770(auStack_28,param_1);
        FUN_00568770(auStack_28,param_1);
        FUN_00568770(auStack_28,param_1);
        uStack_4 = 0xffffffff;
        FUN_005686e0(auStack_28);
        bVar4 = FUN_005f4d40(param_1);
        iVar1 = CONCAT31(extraout_var_10,bVar4);
      }
      bVar4 = iVar1 != 0;
      iVar1 = 0;
      if (bVar4) {
        iVar1 = FUN_00568560(param_1);
      }
    }
  }
  *(undefined4 *)((int)this + 0x30) = 0;
  *(undefined4 *)((int)this + 0x34) = 0;
  *(undefined4 *)((int)this + 0x38) = 0;
  *(undefined4 *)((int)this + 0x3c) = 0;
  *(undefined4 *)((int)this + 0x40) = 0;
  *(undefined4 *)((int)this + 0x44) = 0;
  *(undefined4 *)((int)this + 0x48) = 0;
  *(undefined4 *)((int)this + 0x4c) = 0;
  puVar3 = FUN_005397d0(auStack_28);
  uStack_4 = 1;
  FUN_00539880((void *)((int)this + 0x50),puVar3);
  uStack_4 = 0xffffffff;
  FUN_00539830((int)auStack_28);
  puVar3 = FUN_005397d0(auStack_28);
  uStack_4 = 2;
  FUN_00539880((void *)((int)this + 0x6c),puVar3);
  uStack_4 = 0xffffffff;
  FUN_00539830((int)auStack_28);
  puVar3 = FUN_005397d0(auStack_28);
  uStack_4 = 3;
  FUN_00539880((void *)((int)this + 0x88),puVar3);
  uStack_4 = 0xffffffff;
  FUN_00539830((int)auStack_28);
  ExceptionList = pvStack_c;
  return iVar1;
}

