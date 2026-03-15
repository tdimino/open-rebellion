
void __cdecl FUN_00553c80(uint *param_1,void *param_2)

{
  uint *puVar1;
  int iVar2;
  undefined *puVar3;
  undefined4 local_18 [3];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00649140;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_005f35b0(local_18,&DAT_006b120c);
  local_4 = 0;
  FUN_005f3090(param_2,(int)local_18);
  local_4 = 0xffffffff;
  FUN_005f2ff0(local_18);
  puVar1 = FUN_004ece40(param_1);
  if (puVar1 == (uint *)0x0) {
    FUN_005f35b0(local_18,s_<empty_GameObjKey>_006aaff0);
    local_4 = 7;
    FUN_005f3090(param_2,(int)local_18);
  }
  else {
    iVar2 = FUN_0053efa0(param_1);
    if (iVar2 == 0) {
      FUN_005f35b0(local_18,s_<can_t_find_name,_type___006ab008);
      local_4 = 3;
      FUN_005f30d0(param_2,(int)local_18);
      local_4 = 0xffffffff;
      FUN_005f2ff0(local_18);
      FUN_005f31f0(param_2,*param_1 >> 0x18);
      FUN_005f35b0(local_18,&DAT_006ab004);
      local_4 = 4;
      FUN_005f30d0(param_2,(int)local_18);
      local_4 = 0xffffffff;
      FUN_005f2ff0(local_18);
      FUN_005f35b0(local_18,s__ID__006ab024);
      local_4 = 5;
      FUN_005f30d0(param_2,(int)local_18);
      local_4 = 0xffffffff;
      FUN_005f2ff0(local_18);
      FUN_005f31f0(param_2,*param_1 & 0xffffff);
      FUN_005f35b0(local_18,&DAT_006aa5d0);
      local_4 = 6;
      FUN_005f30d0(param_2,(int)local_18);
    }
    else {
      puVar3 = FUN_004f62d0(iVar2);
      FUN_005f30d0(param_2,(int)puVar3);
      FUN_005f35b0(local_18,s__ID__006ab024);
      local_4 = 1;
      FUN_005f30d0(param_2,(int)local_18);
      local_4 = 0xffffffff;
      FUN_005f2ff0(local_18);
      FUN_005f31f0(param_2,*param_1 & 0xffffff);
      FUN_005f35b0(local_18,&DAT_006aa5d0);
      local_4 = 2;
      FUN_005f30d0(param_2,(int)local_18);
    }
  }
  local_4 = 0xffffffff;
  FUN_005f2ff0(local_18);
  ExceptionList = local_c;
  return;
}

