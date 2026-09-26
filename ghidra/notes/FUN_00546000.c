
bool FUN_00546000(uint *param_1,int param_2,void *param_3)

{
  bool bVar1;
  void *pvVar2;
  uint *puVar3;
  int iVar4;
  int *this;
  bool bVar5;
  int local_78;
  int *local_74;
  undefined4 local_6c [11];
  void *local_40;
  undefined4 local_3c [11];
  void *local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00647680;
  local_c = ExceptionList;
  bVar5 = true;
  if (param_2 != 0) {
    ExceptionList = &local_c;
    pvVar2 = (void *)FUN_00505d40(param_1);
    bVar5 = pvVar2 != (void *)0x0;
    if (pvVar2 != (void *)0x0) {
      puVar3 = FUN_0042d170(pvVar2,&param_2);
      local_4 = 0;
      iVar4 = FUN_004ece60(puVar3);
      local_4 = 0xffffffff;
      FUN_00619730();
      if ((iVar4 != 0) && ((*(byte *)((int)pvVar2 + 0x78) & 8) != 0)) {
        puVar3 = FUN_0042d170(pvVar2,&param_2);
        local_4 = 1;
        this = (int *)FUN_00505970(puVar3);
        local_4 = 0xffffffff;
        local_74 = this;
        FUN_00619730();
        bVar5 = bVar5 && this != (int *)0x0;
        if (this != (int *)0x0) {
          param_2 = 0;
          local_78 = 0;
          param_1 = (uint *)0x0;
          bVar1 = false;
          FUN_00525c60(local_3c,this);
          local_4 = 2;
          FUN_00525930((int)local_3c);
          pvVar2 = param_3;
          while (param_3 = pvVar2, local_10 != (void *)0x0) {
            iVar4 = FUN_00521220(this,local_10,&param_2,&local_78,&param_1);
            if ((iVar4 == 0) || (!bVar5)) {
              bVar5 = false;
            }
            else {
              bVar5 = true;
            }
            if (param_2 == 0) {
              bVar1 = true;
            }
            FUN_005258f0((int)local_3c);
            pvVar2 = param_3;
          }
          if (!bVar1) {
            FUN_00525d10(local_6c,this);
            local_4 = CONCAT31(local_4._1_3_,3);
            FUN_00525930((int)local_6c);
            while (local_40 != (void *)0x0) {
              if ((*(byte *)((int)local_40 + 0x78) & 0x10) != 0) {
                iVar4 = FUN_00534640(local_40,1,pvVar2);
                if ((iVar4 == 0) || (!bVar5)) {
                  bVar5 = false;
                }
                else {
                  bVar5 = true;
                }
              }
              FUN_005258f0((int)local_6c);
              this = local_74;
            }
            local_4._1_3_ = (undefined3)((uint)local_4 >> 8);
            local_4 = CONCAT31(local_4._1_3_,2);
            FUN_00525db0(local_6c);
          }
          iVar4 = FUN_00548120(this,pvVar2);
          if ((iVar4 == 0) || (!bVar5)) {
            bVar5 = false;
          }
          else {
            bVar5 = true;
          }
          local_4 = 0xffffffff;
          FUN_00525d00(local_3c);
        }
      }
    }
  }
  ExceptionList = local_c;
  return bVar5;
}

