
int __cdecl FUN_0040ffd0(int param_1,void *param_2,int param_3)

{
  char cVar1;
  int iVar2;
  char *pcVar3;
  int iVar4;
  int iVar5;
  char *pcVar6;
  uint uVar7;
  uint uVar8;
  int local_24;
  byte local_20 [4];
  undefined1 local_1c;
  undefined4 local_18 [3];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062a4c8;
  local_c = ExceptionList;
  local_24 = 1;
  ExceptionList = &local_c;
  pcVar3 = (char *)FUN_00583c40(param_1);
  FUN_005f3c20(param_2);
  if (*pcVar3 == '\x17') {
    local_1c = 0;
    _strncpy((char *)local_20,pcVar3 + 1,4);
    iVar4 = FUN_00619600(local_20);
    iVar5 = FUN_00619600(&DAT_006a8164);
    iVar2 = param_3;
    if (iVar4 == iVar5) {
      pcVar6 = pcVar3 + 5;
      uVar7 = 5;
      cVar1 = pcVar3[5];
      while (cVar1 != '\0') {
        if (*pcVar6 == '\x17') {
          param_3 = iVar2 << 0x10;
          FUN_005f2f50(local_18);
          uVar8 = uVar7 + 1;
          local_4 = 0;
          switch(pcVar3[uVar8 & 0xffff]) {
          case '1':
            uVar8 = uVar7 + 2;
            if (pcVar3[uVar8 & 0xffff] == '0') {
              param_3 = CONCAT22(param_3._2_2_,0x1150);
            }
            else if (pcVar3[uVar8 & 0xffff] == '1') {
              param_3 = CONCAT22(param_3._2_2_,0x1151);
            }
            break;
          case '2':
            uVar8 = uVar7 + 2;
            cVar1 = pcVar3[uVar8 & 0xffff];
            if (cVar1 == '0') {
              param_3 = CONCAT22(param_3._2_2_,0x1152);
            }
            else if (cVar1 == '1') {
              param_3 = CONCAT22(param_3._2_2_,0x1153);
            }
            else if (cVar1 == '2') {
              param_3 = CONCAT22(param_3._2_2_,0x1154);
            }
            break;
          case '3':
            uVar8 = uVar7 + 2;
            if (pcVar3[uVar8 & 0xffff] == '0') {
              param_3 = CONCAT22(param_3._2_2_,0x1155);
            }
            else if (pcVar3[uVar8 & 0xffff] == '1') {
              param_3 = CONCAT22(param_3._2_2_,0x1156);
            }
            break;
          case '4':
            param_3 = CONCAT22(param_3._2_2_,0x1158);
            local_24 = 3;
          }
          if ((short)param_3 != 0) {
            FUN_005f3010(local_18,&param_3);
          }
          FUN_005f30d0(param_2,(int)local_18);
          local_4 = 0xffffffff;
          FUN_005f2ff0(local_18);
          uVar7 = uVar8;
        }
        else {
          FUN_005f3240(param_2,pcVar6);
        }
        uVar7 = uVar7 + 1;
        pcVar6 = pcVar3 + (uVar7 & 0xffff);
        cVar1 = *pcVar6;
      }
      if (local_24 == 1) {
        local_24 = 0;
      }
    }
    else {
      local_24 = 1;
      if (iVar5 < iVar4) {
        local_24 = 2;
      }
    }
  }
  ExceptionList = local_c;
  return local_24;
}

