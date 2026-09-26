
void * __thiscall
FUN_00403460(void *this,uint param_1,uint param_2,uint param_3,uint param_4,uint param_5,int param_6
            ,int param_7,void *param_8)

{
  int iVar1;
  bool bVar2;
  void *this_00;
  void *pvVar3;
  undefined4 *puVar4;
  void *local_14;
  void *local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006294cd;
  local_c = ExceptionList;
  bVar2 = false;
  ExceptionList = &local_c;
  local_14 = (void *)FUN_00618b70(0x20);
  local_4 = 0;
  if (local_14 == (void *)0x0) {
    param_8 = (void *)0x0;
  }
  else {
    param_8 = FUN_0041be80(local_14,param_8);
  }
  local_4 = 0xffffffff;
  if (param_8 != (void *)0x0) {
    for (this_00 = (void *)thunk_FUN_005f5060((int)this); this_00 != (void *)0x0;
        this_00 = *(void **)((int)this_00 + 0x10)) {
      if (((((param_1 & *(uint *)((int)this_00 + 0x30)) == param_1) &&
           ((param_3 & *(uint *)((int)this_00 + 0x30)) == 0)) &&
          ((param_2 & *(uint *)((int)this_00 + 0x34)) == param_2)) &&
         ((((param_4 & *(uint *)((int)this_00 + 0x34)) == 0 &&
           (iVar1 = *(int *)((int)this_00 + (param_5 & 0xffff) * 4 + 0x40), param_6 <= iVar1)) &&
          (iVar1 <= param_7)))) {
        pvVar3 = (void *)FUN_00618b70(0x18);
        local_4 = 1;
        local_10 = pvVar3;
        if (pvVar3 == (void *)0x0) {
          pvVar3 = (void *)0x0;
        }
        else {
          puVar4 = FUN_00403040(this_00,&local_14);
          bVar2 = true;
          local_4 = CONCAT31(local_4._1_3_,2);
          pvVar3 = FUN_0041c360(pvVar3,puVar4,iVar1);
        }
        local_4 = 1;
        if (bVar2) {
          bVar2 = false;
          FUN_00619730();
        }
        local_4 = 0xffffffff;
        FUN_0041c070(param_8,pvVar3);
      }
    }
    FUN_0041c230((int)param_8);
  }
  ExceptionList = local_c;
  return param_8;
}

